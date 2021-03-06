use hashbrown::HashMap;
use rayon::{
    prelude::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use signal_hook;
use std::{
    cmp::Ord,
    error::Error,
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::config;

pub struct Processor {
    config: config::Config,
}

impl Processor {
    pub fn init(config: config::Config) -> Processor {
        return Processor { config: config };
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let inp = &self.config.input;
        let sig_pipe = self.watch_sig_pipe()?;
        let reader = self.create_reader(&inp)?;
        let counter = self.count_items(reader)?;

        let mut counts: Vec<_> = counter.par_iter().collect();
        self.sort_counts(&mut counts);

        let mut n = self.config.max_items as usize;
        if n == 0 {
            n = counts.len()
        }

        let stdout = stdout();
        let handle = stdout.lock();

        self.output_counts(handle, counts, n, sig_pipe)?;
        Ok(())
    }

    fn create_reader(&self, input: &Option<String>) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
        let reader: Box<dyn BufRead> = match input {
            Some(file_name) => Box::new(BufReader::new(File::open(file_name)?)),
            None => Box::new(BufReader::new(stdin())),
        };
        Ok(reader)
    }

    fn count_items(
        &self,
        mut reader: Box<dyn BufRead>,
    ) -> Result<HashMap<Vec<u8>, u64>, Box<dyn Error>> {
        let mut counter: HashMap<_, u64> = Default::default();

        let mut buf = Vec::with_capacity(64);
        while let Ok(n) = reader.read_until(b'\n', &mut buf) {
            // trim trailing newline
            if n == 0 {
                break;
            } else if buf[n - 1] == b'\n' {
                let n_end = if n > 1 && buf[n - 2] == b'\r' {
                    n - 2
                } else {
                    n - 1
                };
                buf.truncate(n_end);
            }
            match counter.get_mut(&buf) {
                Some(count) => {
                    *count += 1;
                }
                None => {
                    counter.insert(buf.to_owned(), 1);
                }
            };
            buf.clear();
        }

        Ok(counter)
    }

    fn sort_counts<S: Ord + Sync, T: Ord + Sync>(&self, counts: &mut Vec<(&S, &T)>) {
        match self.config.sort_by {
            config::SortingOrder::Key => {
                counts.par_sort_unstable_by(|k, v| k.0.cmp(v.0).then(k.1.cmp(k.1).reverse()))
            }
            config::SortingOrder::Count => {
                counts.par_sort_unstable_by(|k, v| k.1.cmp(v.1).reverse().then(k.0.cmp(v.0)))
            }
            config::SortingOrder::None => (),
        }
    }

    fn output_counts<T: Write>(
        &self,
        mut io: T,
        counts: Vec<(&Vec<u8>, &u64)>,
        n: usize,
        sig_pipe: Arc<AtomicBool>,
    ) -> Result<(), Box<dyn Error>> {
        for (key, count) in counts.into_iter().take(n) {
            // print either with count
            if self.config.count == true {
                if count >= &self.config.threshold {
                    writeln!(
                        io,
                        "{}{}{}",
                        count,
                        self.config.separator,
                        String::from_utf8(key.to_owned())?
                    )?;
                }
            // or without
            } else {
                if count >= &self.config.threshold {
                    writeln!(io, "{}", String::from_utf8(key.to_owned())?)?;
                }
            }
            if sig_pipe.load(Ordering::Relaxed) {
                break;
            }
        }
        Ok(())
    }

    fn watch_sig_pipe(&self) -> Result<Arc<AtomicBool>, Box<dyn Error>> {
        let sig_pipe = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(signal_hook::SIGPIPE, Arc::clone(&sig_pipe))?;
        Ok(sig_pipe)
    }
}
