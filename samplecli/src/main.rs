use anyhow::{bail, ensure, Context, Result};

use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};




struct RpnCaluculator(bool);

impl RpnCaluculator{
    pub fn new(verbose: bool) -> Self{
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32>{
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32>{
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop(){
            pos += 1;

            if let Ok(x) = token.parse::<i32>(){
                stack.push(x);
            }else{
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;

                let res = match token{
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "&" => x % y,
                    _ => bail!("invalid token{}", pos),
                };
                stack.push(res);
            }

            if self.0{
                println!("{:?}{:?}", tokens, stack);
            }
        }

        ensure!(stack.len() == 1, "invalid syntax");
        
        Ok(stack[0])
    }
}

#[derive(Clap, Debug)]

#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"
)]

struct Opts{
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() -> Result<()>{
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file{
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    }else{
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()>{
    let calc = RpnCaluculator::new(verbose);

    for line in reader.lines(){
        let line = line?;
        match calc.eval(&line){
            Ok(answer) => println!("{}", answer),
            Err(e) => println!("{:#?}", e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_ok(){
        let calc = RpnCaluculator::new(false);
        assert_eq!(calc.eval("5"), 5);
        assert_eq!(calc.eval("2 3 +"), 5);
    }
}

#[test]
#[should_panic]
fn test_ng(){
    let calc = RpnCaluculator::new(false);
    calc.eval("1 1 ^");
}