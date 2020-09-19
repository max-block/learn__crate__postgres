use postgres::{Client, Error, NoTls};

fn init_tables(client: &mut Client) -> Result<(), Error> {
    client.batch_execute("\
drop table if exists data;
create table data (
    id serial primary key,
    name text not null,
    value int not null,
    created_at timestamp with time zone not null default now()
);
    ")
}

fn insert_data(client: &mut Client) -> Result<(), Error> {
    client.execute("insert into data (name, value) values($1, $2)", &[&"n1".to_string(), &17])?;
    client.execute("insert into data (name, value) values($1, $2)", &[&"n2".to_string(), &13])?;

    Ok(())
}

fn query_one(client: &mut Client) -> Result<(), Error> {
    let res = client.query_one("select * from data where name=$1", &[&"n1".to_string()])?;

    let name: String = res.get("name");
    let value: i32 = res.get("value");
    println!("name={name}, value={value}", name = name, value = value);


    Ok(())
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect("host=localhost user=max dbname=learn__crate__postgres", NoTls)?;

    init_tables(&mut client)?;
    insert_data(&mut client)?;
    query_one(&mut client)?;

    Ok(())
}
