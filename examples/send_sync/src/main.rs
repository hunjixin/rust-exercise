
fn is_send<T>()
where T:Send
{}

fn is_sync<T>()
where T:Sync
{}


struct TestT<'a, DBConfig> 
where DBConfig: Send
{
    log_level: &'a str,
    db: DBConfig,
}


fn main() {
    is_send::<TestT<String>>()
}
