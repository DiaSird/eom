pub fn write_csv(cnf: &Config, cdata: &CalcData)
        -> Result<(), Box<dyn error::Error>> {

    let file_name: String = format!("{}/{:08}.csv", &cnf.dir_name, &cdata.output_num);
    let file = File::create(file_name).unwrap();

    let mut w = BufWriter::new(file);
    // write!(file, "header")
    write!(w, "x,y,u\n").unwrap();

    for i in 0..cnf.nx {
        for j in 0..cnf.ny {
            let s = format!(
                "{},{},{}\n",
                &cdata.x[i],
                &cdata.y[j],
                &cdata.u[(i, j, cnf.nz / 2)],
            );
            write!(w, "{}", s).unwrap();
        }
    }
    w.flush().unwrap();
    Ok(())
}