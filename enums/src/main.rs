enum IpAddrKind
{
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr
{
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
