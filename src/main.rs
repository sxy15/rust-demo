fn main() {
    {
        let x;
        {
            let y = 5;
            // x = &y;
            x = y; 
        }

        println!("{}", x);
    }
}