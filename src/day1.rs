use std::fs;

fn main()
{
    let file = fs::read_to_string("1.txt").unwrap();

    let mut f = 0;
    let mut seen = vec![0];

    'out: loop
    {
        for line in file.lines()
        {
            let delta = line.replace("+", "");
            let delta: i32 = delta.parse().unwrap();
            f = f + delta;
            if seen.contains(&f)
            {
                println!("{}", f);
                break 'out;
            }
            else
            {
                seen.push(f);
            }
        }
    }
    println!("{}", f);
}
