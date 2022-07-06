extern crate machine_uid;

pub struct exampleArgs {
    x: i32,
}

pub struct exampleReply {
    y: i32,
}

pub fn coorinatrorSock() -> String {
    let mut s: String = "/var/tmp/mr-";
    let id: String = machine_uid::get().unwrap();

    s += id;
}

#[cfg(test)]
mod tests {}
