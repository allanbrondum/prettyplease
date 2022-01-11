#[cfg(all(test, not(target_os = "emscripten")))]
mod tests;
use crate::cmp::Ordering;
use crate::fmt::{self, Write as FmtWrite};
use crate::hash;
use crate::io::Write as IoWrite;
use crate::mem::transmute;
use crate::sys::net::netc as c;
use crate::sys_common::{AsInner, FromInner, IntoInner};
#[doc = " An IP address, either IPv4 or IPv6."]
#[doc = ""]
#[doc =
  " This enum can contain either an [`Ipv4Addr`] or an [`Ipv6Addr`], see their"]
#[doc = " respective documentation for more details."]
#[doc = ""]
#[doc =
  " The size of an `IpAddr` instance may vary depending on the target operating"]
#[doc = " system."]
#[doc = ""]
#[doc = " # Examples"]
#[doc = ""]
#[doc = " ```"]
#[doc = " use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};"]
#[doc = ""]
#[doc = " let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));"]
#[doc =
  " let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));"]
#[doc = ""]
#[doc = " assert_eq!(\"127.0.0.1\".parse(), Ok(localhost_v4));"]
#[doc = " assert_eq!(\"::1\".parse(), Ok(localhost_v6));"]
#[doc = ""]
#[doc = " assert_eq!(localhost_v4.is_ipv6(), false);"]
#[doc = " assert_eq!(localhost_v4.is_ipv4(), true);"]
#[doc = " ```"]
#[stable(feature = "ip_addr", since = "1.7.0")]
#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum IpAddr {

    #[doc = " An IPv4 address."]
    #[stable(feature = "ip_addr", since = "1.7.0")]
    V4(
       #[stable(feature = "ip_addr", since = "1.7.0")]
       Ipv4Addr),

    #[doc = " An IPv6 address."]
    #[stable(feature = "ip_addr", since = "1.7.0")]
    V6(
       #[stable(feature = "ip_addr", since = "1.7.0")]
       Ipv6Addr),
}
#[doc = " An IPv4 address."]
#[doc = ""]
#[doc = " IPv4 addresses are defined as 32-bit integers in [IETF RFC 791]."]
#[doc = " They are usually represented as four octets."]
#[doc = ""]
#[doc =
  " See [`IpAddr`] for a type encompassing both IPv4 and IPv6 addresses."]
#[doc = ""]
#[doc =
  " The size of an `Ipv4Addr` struct may vary depending on the target operating"]
#[doc = " system."]
#[doc = ""]
#[doc = " [IETF RFC 791]: https://tools.ietf.org/html/rfc791"]
#[doc = ""]
#[doc = " # Textual representation"]
#[doc = ""]
#[doc =
  " `Ipv4Addr` provides a [`FromStr`] implementation. The four octets are in decimal"]
#[doc =
  " notation, divided by `.` (this is called \"dot-decimal notation\")."]
#[doc =
  " Notably, octal numbers (which are indicated with a leading `0`) and hexadecimal numbers (which"]
#[doc =
  " are indicated with a leading `0x`) are not allowed per [IETF RFC 6943]."]
#[doc = ""]
#[doc = " [IETF RFC 6943]: https://tools.ietf.org/html/rfc6943#section-3.1.1"]
#[doc = " [`FromStr`]: crate::str::FromStr"]
#[doc = ""]
#[doc = " # Examples"]
#[doc = ""]
#[doc = " ```"]
#[doc = " use std::net::Ipv4Addr;"]
#[doc = ""]
#[doc = " let localhost = Ipv4Addr::new(127, 0, 0, 1);"]
#[doc = " assert_eq!(\"127.0.0.1\".parse(), Ok(localhost));"]
#[doc = " assert_eq!(localhost.is_loopback(), true);"]
#[doc =
  " assert!(\"012.004.002.000\".parse::<Ipv4Addr>().is_err()); // all octets are in octal"]
#[doc =
  " assert!(\"0000000.0.0.0\".parse::<Ipv4Addr>().is_err()); // first octet is a zero in octal"]
#[doc =
  " assert!(\"0xcb.0x0.0x71.0x00\".parse::<Ipv4Addr>().is_err()); // all octets are in hex"]
#[doc = " ```"]
#[derive(Copy)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Ipv4Addr {
    inner: c::in_addr,
}
#[doc = " An IPv6 address."]
#[doc = ""]
#[doc = " IPv6 addresses are defined as 128-bit integers in [IETF RFC 4291]."]
#[doc = " They are usually represented as eight 16-bit segments."]
#[doc = ""]
#[doc =
  " The size of an `Ipv6Addr` struct may vary depending on the target operating"]
#[doc = " system."]
#[doc = ""]
#[doc = " [IETF RFC 4291]: https://tools.ietf.org/html/rfc4291"]
#[doc = ""]
#[doc = " # Embedding IPv4 Addresses"]
#[doc = ""]
#[doc =
  " See [`IpAddr`] for a type encompassing both IPv4 and IPv6 addresses."]
#[doc = ""]
#[doc =
  " To assist in the transition from IPv4 to IPv6 two types of IPv6 addresses that embed an IPv4 address were defined:"]
#[doc =
  " IPv4-compatible and IPv4-mapped addresses. Of these IPv4-compatible addresses have been officially deprecated."]
#[doc = ""]
#[doc =
  " Both types of addresses are not assigned any special meaning by this implementation,"]
#[doc =
  " other than what the relevant standards prescribe. This means that an address like `::ffff:127.0.0.1`,"]
#[doc =
  " while representing an IPv4 loopback address, is not itself an IPv6 loopback address; only `::1` is."]
#[doc =
  " To handle these so called \"IPv4-in-IPv6\" addresses, they have to first be converted to their canonical IPv4 address."]
#[doc = ""]
#[doc = " ### IPv4-Compatible IPv6 Addresses"]
#[doc = ""]
#[doc =
  " IPv4-compatible IPv6 addresses are defined in [IETF RFC 4291 Section 2.5.5.1], and have been officially deprecated."]
#[doc =
  " The RFC describes the format of an \"IPv4-Compatible IPv6 address\" as follows:"]
#[doc = ""]
#[doc = " ```text"]
#[doc =
  " |                80 bits               | 16 |      32 bits        |"]
#[doc =
  " +--------------------------------------+--------------------------+"]
#[doc =
  " |0000..............................0000|0000|    IPv4 address     |"]
#[doc =
  " +--------------------------------------+----+---------------------+"]
#[doc = " ```"]
#[doc =
  " So `::a.b.c.d` would be an IPv4-compatible IPv6 address representing the IPv4 address `a.b.c.d`."]
#[doc = ""]
#[doc =
  " To convert from an IPv4 address to an IPv4-compatible IPv6 address, use [`Ipv4Addr::to_ipv6_compatible`]."]
#[doc =
  " Use [`Ipv6Addr::to_ipv4`] to convert an IPv4-compatible IPv6 address to the canonical IPv4 address."]
#[doc = ""]
#[doc =
  " [IETF RFC 4291 Section 2.5.5.1]: https://datatracker.ietf.org/doc/html/rfc4291#section-2.5.5.1"]
#[doc = ""]
#[doc = " ### IPv4-Mapped IPv6 Addresses"]
#[doc = ""]
#[doc =
  " IPv4-mapped IPv6 addresses are defined in [IETF RFC 4291 Section 2.5.5.2]."]
#[doc =
  " The RFC describes the format of an \"IPv4-Mapped IPv6 address\" as follows:"]
#[doc = ""]
#[doc = " ```text"]
#[doc =
  " |                80 bits               | 16 |      32 bits        |"]
#[doc =
  " +--------------------------------------+--------------------------+"]
#[doc =
  " |0000..............................0000|FFFF|    IPv4 address     |"]
#[doc =
  " +--------------------------------------+----+---------------------+"]
#[doc = " ```"]
#[doc =
  " So `::ffff:a.b.c.d` would be an IPv4-mapped IPv6 address representing the IPv4 address `a.b.c.d`."]
#[doc = ""]
#[doc =
  " To convert from an IPv4 address to an IPv4-mapped IPv6 address, use [`Ipv4Addr::to_ipv6_mapped`]."]
#[doc =
  " Use [`Ipv6Addr::to_ipv4`] to convert an IPv4-mapped IPv6 address to the canonical IPv4 address."]
#[doc = ""]
#[doc =
  " [IETF RFC 4291 Section 2.5.5.2]: https://datatracker.ietf.org/doc/html/rfc4291#section-2.5.5.2"]
#[doc = ""]
#[doc = " # Textual representation"]
#[doc = ""]
#[doc =
  " `Ipv6Addr` provides a [`FromStr`] implementation. There are many ways to represent"]
#[doc =
  " an IPv6 address in text, but in general, each segments is written in hexadecimal"]
#[doc =
  " notation, and segments are separated by `:`. For more information, see"]
#[doc = " [IETF RFC 5952]."]
#[doc = ""]
#[doc = " [`FromStr`]: crate::str::FromStr"]
#[doc = " [IETF RFC 5952]: https://tools.ietf.org/html/rfc5952"]
#[doc = ""]
#[doc = " # Examples"]
#[doc = ""]
#[doc = " ```"]
#[doc = " use std::net::Ipv6Addr;"]
#[doc = ""]
#[doc = " let localhost = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);"]
#[doc = " assert_eq!(\"::1\".parse(), Ok(localhost));"]
#[doc = " assert_eq!(localhost.is_loopback(), true);"]
#[doc = " ```"]
#[derive(Copy)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Ipv6Addr {
    inner: c::in6_addr,
}
#[doc =
  " Scope of an [IPv6 multicast address] as defined in [IETF RFC 7346 section 2]."]
#[doc = ""]
#[doc = " # Stability Guarantees"]
#[doc = ""]
#[doc = " Not all possible values for a multicast scope have been assigned."]
#[doc =
  " Future RFCs may introduce new scopes, which will be added as variants to this enum;"]
#[doc = " because of this the enum is marked as `#[non_exhaustive]`."]
#[doc = ""]
#[doc = " # Examples"]
#[doc = " ```"]
#[doc = " #![feature(ip)]"]
#[doc = ""]
#[doc = " use std::net::Ipv6Addr;"]
#[doc = " use std::net::Ipv6MulticastScope::*;"]
#[doc = ""]
#[doc = " // An IPv6 multicast address with global scope (`ff0e::`)."]
#[doc = " let address = Ipv6Addr::new(0xff0e, 0, 0, 0, 0, 0, 0, 0);"]
#[doc = ""]
#[doc = " // Will print \"Global scope\"."]
#[doc = " match address.multicast_scope() {"]
#[doc = "     Some(InterfaceLocal) => println!(\"Interface-Local scope\"),"]
#[doc = "     Some(LinkLocal) => println!(\"Link-Local scope\"),"]
#[doc = "     Some(RealmLocal) => println!(\"Realm-Local scope\"),"]
#[doc = "     Some(AdminLocal) => println!(\"Admin-Local scope\"),"]
#[doc = "     Some(SiteLocal) => println!(\"Site-Local scope\"),"]
#[doc =
  "     Some(OrganizationLocal) => println!(\"Organization-Local scope\"),"]
#[doc = "     Some(Global) => println!(\"Global scope\"),"]
#[doc = "     Some(_) => println!(\"Unknown scope\"),"]
#[doc = "     None => println!(\"Not a multicast address!\")"]
#[doc = " }"]
#[doc = ""]
#[doc = " ```"]
#[doc = ""]
#[doc = " [IPv6 multicast address]: Ipv6Addr"]
#[doc =
  " [IETF RFC 7346 section 2]: https://tools.ietf.org/html/rfc7346#section-2"]
#[derive(Copy, PartialEq, Eq, Clone, Hash, Debug)]
#[unstable(feature = "ip", issue = "27709")]
#[non_exhaustive]
pub enum Ipv6MulticastScope {

    #[doc = " Interface-Local scope."]
    InterfaceLocal,

    #[doc = " Link-Local scope."]
    LinkLocal,

    #[doc = " Realm-Local scope."]
    RealmLocal,

    #[doc = " Admin-Local scope."]
    AdminLocal,

    #[doc = " Site-Local scope."]
    SiteLocal,

    #[doc = " Organization-Local scope."]
    OrganizationLocal,

    #[doc = " Global scope."]
    Global,
}
impl IpAddr {
    #[doc = " Returns [`true`] for the special 'unspecified' address."]
    #[doc = ""]
    #[doc = " See the documentation for [`Ipv4Addr::is_unspecified()`] and"]
    #[doc = " [`Ipv6Addr::is_unspecified()`] for more details."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};"]
    #[doc = ""]
    #[doc =
      " assert_eq!(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)).is_unspecified(), true);"]
    #[doc =
      " assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)).is_unspecified(), true);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ip", since = "1.50.0")]
    #[stable(feature = "ip_shared", since = "1.12.0")]
    #[must_use]
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_unspecified(),
            IpAddr::V6(ip) => ip.is_unspecified(),
        }
    }
    #[doc = " Returns [`true`] if this is a loopback address."]
    #[doc = ""]
    #[doc = " See the documentation for [`Ipv4Addr::is_loopback()`] and"]
    #[doc = " [`Ipv6Addr::is_loopback()`] for more details."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};"]
    #[doc = ""]
    #[doc =
      " assert_eq!(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)).is_loopback(), true);"]
    #[doc =
      " assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0x1)).is_loopback(), true);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ip", since = "1.50.0")]
    #[stable(feature = "ip_shared", since = "1.12.0")]
    #[must_use]
    #[inline]
    pub const fn is_loopback(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_loopback(),
            IpAddr::V6(ip) => ip.is_loopback(),
        }
    }
    #[doc =
      " Returns [`true`] if the address appears to be globally routable."]
    #[doc = ""]
    #[doc = " See the documentation for [`Ipv4Addr::is_global()`] and"]
    #[doc = " [`Ipv6Addr::is_global()`] for more details."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = ""]
    #[doc = " use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};"]
    #[doc = ""]
    #[doc =
      " assert_eq!(IpAddr::V4(Ipv4Addr::new(80, 9, 12, 3)).is_global(), true);"]
    #[doc =
      " assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0x1c9, 0, 0, 0xafc8, 0, 0x1)).is_global(), true);"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ip", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn is_global(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_global(),
            IpAddr::V6(ip) => ip.is_global(),
        }
    }
    #[doc = " Returns [`true`] if this is a multicast address."]
    #[doc = ""]
    #[doc = " See the documentation for [`Ipv4Addr::is_multicast()`] and"]
    #[doc = " [`Ipv6Addr::is_multicast()`] for more details."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};"]
    #[doc = ""]
    #[doc =
      " assert_eq!(IpAddr::V4(Ipv4Addr::new(224, 254, 0, 0)).is_multicast(), true);"]
    #[doc =
      " assert_eq!(IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0)).is_multicast(), true);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ip", since = "1.50.0")]
    #[stable(feature = "ip_shared", since = "1.12.0")]
    #[must_use]
    #[inline]
    pub const fn is_multicast(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_multicast(),
            IpAddr::V6(ip) => ip.is_multicast(),
        }
    }
    #[doc =
      " Returns [`true`] if this address is in a range designated for documentation."]
    #[doc = ""]
    #[doc = " See the documentation for [`Ipv4Addr::is_documentation()`] and"]
    #[doc = " [`Ipv6Addr::is_documentation()`] for more details."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = ""]
    #[doc = " use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};"]
    #[doc = ""]
    #[doc =
      " assert_eq!(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 6)).is_documentation(), true);"]
    #[doc = " assert_eq!("]
    #[doc =
      "     IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0)).is_documentation(),"]
    #[doc = "     true"]
    #[doc = " );"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ip", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn is_documentation(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_documentation(),
            IpAddr::V6(ip) => ip.is_documentation(),
        }
    }
    #[doc =
      " Returns [`true`] if this address is in a range designated for benchmarking."]
    #[doc = ""]
    #[doc = " See the documentation for [`Ipv4Addr::is_benchmarking()`] and"]
    #[doc = " [`Ipv6Addr::is_benchmarking()`] for more details."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = ""]
    #[doc = " use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};"]
    #[doc = ""]
    #[doc =
      " assert_eq!(IpAddr::V4(Ipv4Addr::new(198, 19, 255, 255)).is_benchmarking(), true);"]
    #[doc =
      " assert_eq!(IpAddr::V6(Ipv6Addr::new(0x2001, 0x2, 0, 0, 0, 0, 0, 0)).is_benchmarking(), true);"]
    #[doc = " ```"]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn is_benchmarking(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_benchmarking(),
            IpAddr::V6(ip) => ip.is_benchmarking(),
        }
    }
    #[doc =
      " Returns [`true`] if this address is an [`IPv4` address], and [`false`]"]
    #[doc = " otherwise."]
    #[doc = ""]
    #[doc = " [`IPv4` address]: IpAddr::V4"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};"]
    #[doc = ""]
    #[doc =
      " assert_eq!(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 6)).is_ipv4(), true);"]
    #[doc =
      " assert_eq!(IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0)).is_ipv4(), false);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ip", since = "1.50.0")]
    #[stable(feature = "ipaddr_checker", since = "1.16.0")]
    #[must_use]
    #[inline]
    pub const fn is_ipv4(&self) -> bool { matches!(self, IpAddr :: V4(_)) }
    #[doc =
      " Returns [`true`] if this address is an [`IPv6` address], and [`false`]"]
    #[doc = " otherwise."]
    #[doc = ""]
    #[doc = " [`IPv6` address]: IpAddr::V6"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};"]
    #[doc = ""]
    #[doc =
      " assert_eq!(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 6)).is_ipv6(), false);"]
    #[doc =
      " assert_eq!(IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0)).is_ipv6(), true);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ip", since = "1.50.0")]
    #[stable(feature = "ipaddr_checker", since = "1.16.0")]
    #[must_use]
    #[inline]
    pub const fn is_ipv6(&self) -> bool { matches!(self, IpAddr :: V6(_)) }
    #[doc =
      " Converts this address to an `IpAddr::V4` if it is an IPv4-mapped IPv6 addresses, otherwise it"]
    #[doc = " return `self` as-is."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = " use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};"]
    #[doc = ""]
    #[doc =
      " assert_eq!(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)).to_canonical().is_loopback(), true);"]
    #[doc =
      " assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0x7f00, 0x1)).is_loopback(), false);"]
    #[doc =
      " assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0x7f00, 0x1)).to_canonical().is_loopback(), true);"]
    #[doc = " ```"]
    #[inline]
    #[must_use =
      "this returns the result of the operation, \
                  without modifying the original"]
    #[rustc_const_unstable(feature = "const_ip", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    pub const fn to_canonical(&self) -> IpAddr {
        match self {
            &v4 @ IpAddr::V4(_) => v4,
            IpAddr::V6(v6) => v6.to_canonical(),
        }
    }
}
impl Ipv4Addr {
    #[doc = " Creates a new IPv4 address from four eight-bit octets."]
    #[doc = ""]
    #[doc = " The result will represent the IP address `a`.`b`.`c`.`d`."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc = " let addr = Ipv4Addr::new(127, 0, 0, 1);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv4", since = "1.32.0")]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use]
    #[inline]
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        Ipv4Addr{inner:
                     c::in_addr{s_addr: u32::from_ne_bytes([a, b, c, d]),},}
    }
    #[doc =
      " An IPv4 address with the address pointing to localhost: `127.0.0.1`"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc = " let addr = Ipv4Addr::LOCALHOST;"]
    #[doc = " assert_eq!(addr, Ipv4Addr::new(127, 0, 0, 1));"]
    #[doc = " ```"]
    #[stable(feature = "ip_constructors", since = "1.30.0")]
    pub const LOCALHOST: Self = Ipv4Addr::new(127, 0, 0, 1);
    #[doc = " An IPv4 address representing an unspecified address: `0.0.0.0`"]
    #[doc = ""]
    #[doc =
      " This corresponds to the constant `INADDR_ANY` in other languages."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc = " let addr = Ipv4Addr::UNSPECIFIED;"]
    #[doc = " assert_eq!(addr, Ipv4Addr::new(0, 0, 0, 0));"]
    #[doc = " ```"]
    #[doc(alias = "INADDR_ANY")]
    #[stable(feature = "ip_constructors", since = "1.30.0")]
    pub const UNSPECIFIED: Self = Ipv4Addr::new(0, 0, 0, 0);
    #[doc =
      " An IPv4 address representing the broadcast address: `255.255.255.255`"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc = " let addr = Ipv4Addr::BROADCAST;"]
    #[doc = " assert_eq!(addr, Ipv4Addr::new(255, 255, 255, 255));"]
    #[doc = " ```"]
    #[stable(feature = "ip_constructors", since = "1.30.0")]
    pub const BROADCAST: Self = Ipv4Addr::new(255, 255, 255, 255);
    #[doc = " Returns the four eight-bit integers that make up this address."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc = " let addr = Ipv4Addr::new(127, 0, 0, 1);"]
    #[doc = " assert_eq!(addr.octets(), [127, 0, 0, 1]);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv4", since = "1.50.0")]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use]
    #[inline]
    pub const fn octets(&self) -> [u8; 4] { self.inner.s_addr.to_ne_bytes() }
    #[doc =
      " Returns [`true`] for the special 'unspecified' address (`0.0.0.0`)."]
    #[doc = ""]
    #[doc =
      " This property is defined in _UNIX Network Programming, Second Edition_,"]
    #[doc = " W. Richard Stevens, p. 891; see also [ip7]."]
    #[doc = ""]
    #[doc = " [ip7]: https://man7.org/linux/man-pages/man7/ip.7.html"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc = " assert_eq!(Ipv4Addr::new(0, 0, 0, 0).is_unspecified(), true);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(45, 22, 13, 197).is_unspecified(), false);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv4", since = "1.32.0")]
    #[stable(feature = "ip_shared", since = "1.12.0")]
    #[must_use]
    #[inline]
    pub const fn is_unspecified(&self) -> bool { self.inner.s_addr == 0 }
    #[doc =
      " Returns [`true`] if this is a loopback address (`127.0.0.0/8`)."]
    #[doc = ""]
    #[doc = " This property is defined by [IETF RFC 1122]."]
    #[doc = ""]
    #[doc = " [IETF RFC 1122]: https://tools.ietf.org/html/rfc1122"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc = " assert_eq!(Ipv4Addr::new(127, 0, 0, 1).is_loopback(), true);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(45, 22, 13, 197).is_loopback(), false);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv4", since = "1.50.0")]
    #[stable(since = "1.7.0", feature = "ip_17")]
    #[must_use]
    #[inline]
    pub const fn is_loopback(&self) -> bool { self.octets()[0] == 127 }
    #[doc = " Returns [`true`] if this is a private address."]
    #[doc = ""]
    #[doc =
      " The private address ranges are defined in [IETF RFC 1918] and include:"]
    #[doc = ""]
    #[doc = "  - `10.0.0.0/8`"]
    #[doc = "  - `172.16.0.0/12`"]
    #[doc = "  - `192.168.0.0/16`"]
    #[doc = ""]
    #[doc = " [IETF RFC 1918]: https://tools.ietf.org/html/rfc1918"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc = " assert_eq!(Ipv4Addr::new(10, 0, 0, 1).is_private(), true);"]
    #[doc = " assert_eq!(Ipv4Addr::new(10, 10, 10, 10).is_private(), true);"]
    #[doc = " assert_eq!(Ipv4Addr::new(172, 16, 10, 10).is_private(), true);"]
    #[doc = " assert_eq!(Ipv4Addr::new(172, 29, 45, 14).is_private(), true);"]
    #[doc = " assert_eq!(Ipv4Addr::new(172, 32, 0, 2).is_private(), false);"]
    #[doc = " assert_eq!(Ipv4Addr::new(192, 168, 0, 2).is_private(), true);"]
    #[doc = " assert_eq!(Ipv4Addr::new(192, 169, 0, 2).is_private(), false);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv4", since = "1.50.0")]
    #[stable(since = "1.7.0", feature = "ip_17")]
    #[must_use]
    #[inline]
    pub const fn is_private(&self) -> bool {
        match self.octets() {
            [10, ..] => true,
            [172, b, ..] if b >= 16 && b <= 31 => true,
            [192, 168, ..] => true,
            _ => false,
        }
    }
    #[doc =
      " Returns [`true`] if the address is link-local (`169.254.0.0/16`)."]
    #[doc = ""]
    #[doc = " This property is defined by [IETF RFC 3927]."]
    #[doc = ""]
    #[doc = " [IETF RFC 3927]: https://tools.ietf.org/html/rfc3927"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv4Addr::new(169, 254, 0, 0).is_link_local(), true);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(169, 254, 10, 65).is_link_local(), true);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(16, 89, 10, 65).is_link_local(), false);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv4", since = "1.50.0")]
    #[stable(since = "1.7.0", feature = "ip_17")]
    #[must_use]
    #[inline]
    pub const fn is_link_local(&self) -> bool {
        matches!(self.octets(), [169, 254, ..])
    }
    #[doc =
      " Returns [`true`] if the address appears to be globally routable."]
    #[doc = " See [iana-ipv4-special-registry][ipv4-sr]."]
    #[doc = ""]
    #[doc = " The following return [`false`]:"]
    #[doc = ""]
    #[doc = " - private addresses (see [`Ipv4Addr::is_private()`])"]
    #[doc = " - the loopback address (see [`Ipv4Addr::is_loopback()`])"]
    #[doc = " - the link-local address (see [`Ipv4Addr::is_link_local()`])"]
    #[doc = " - the broadcast address (see [`Ipv4Addr::is_broadcast()`])"]
    #[doc =
      " - addresses used for documentation (see [`Ipv4Addr::is_documentation()`])"]
    #[doc =
      " - the unspecified address (see [`Ipv4Addr::is_unspecified()`]), and the whole"]
    #[doc = "   `0.0.0.0/8` block"]
    #[doc = " - addresses reserved for future protocols, except"]
    #[doc = " `192.0.0.9/32` and `192.0.0.10/32` which are globally routable"]
    #[doc =
      " - addresses reserved for future use (see [`Ipv4Addr::is_reserved()`]"]
    #[doc = " - addresses reserved for networking devices benchmarking (see"]
    #[doc = " [`Ipv4Addr::is_benchmarking()`])"]
    #[doc = ""]
    #[doc =
      " [ipv4-sr]: https://www.iana.org/assignments/iana-ipv4-special-registry/iana-ipv4-special-registry.xhtml"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = ""]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc = " // private addresses are not global"]
    #[doc = " assert_eq!(Ipv4Addr::new(10, 254, 0, 0).is_global(), false);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(192, 168, 10, 65).is_global(), false);"]
    #[doc = " assert_eq!(Ipv4Addr::new(172, 16, 10, 65).is_global(), false);"]
    #[doc = ""]
    #[doc = " // the 0.0.0.0/8 block is not global"]
    #[doc = " assert_eq!(Ipv4Addr::new(0, 1, 2, 3).is_global(), false);"]
    #[doc = " // in particular, the unspecified address is not global"]
    #[doc = " assert_eq!(Ipv4Addr::new(0, 0, 0, 0).is_global(), false);"]
    #[doc = ""]
    #[doc = " // the loopback address is not global"]
    #[doc = " assert_eq!(Ipv4Addr::new(127, 0, 0, 1).is_global(), false);"]
    #[doc = ""]
    #[doc = " // link local addresses are not global"]
    #[doc = " assert_eq!(Ipv4Addr::new(169, 254, 45, 1).is_global(), false);"]
    #[doc = ""]
    #[doc = " // the broadcast address is not global"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(255, 255, 255, 255).is_global(), false);"]
    #[doc = ""]
    #[doc =
      " // the address space designated for documentation is not global"]
    #[doc = " assert_eq!(Ipv4Addr::new(192, 0, 2, 255).is_global(), false);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(198, 51, 100, 65).is_global(), false);"]
    #[doc = " assert_eq!(Ipv4Addr::new(203, 0, 113, 6).is_global(), false);"]
    #[doc = ""]
    #[doc = " // shared addresses are not global"]
    #[doc = " assert_eq!(Ipv4Addr::new(100, 100, 0, 0).is_global(), false);"]
    #[doc = ""]
    #[doc = " // addresses reserved for protocol assignment are not global"]
    #[doc = " assert_eq!(Ipv4Addr::new(192, 0, 0, 0).is_global(), false);"]
    #[doc = " assert_eq!(Ipv4Addr::new(192, 0, 0, 255).is_global(), false);"]
    #[doc = ""]
    #[doc = " // addresses reserved for future use are not global"]
    #[doc = " assert_eq!(Ipv4Addr::new(250, 10, 20, 30).is_global(), false);"]
    #[doc = ""]
    #[doc =
      " // addresses reserved for network devices benchmarking are not global"]
    #[doc = " assert_eq!(Ipv4Addr::new(198, 18, 0, 0).is_global(), false);"]
    #[doc = ""]
    #[doc = " // All the other addresses are global"]
    #[doc = " assert_eq!(Ipv4Addr::new(1, 1, 1, 1).is_global(), true);"]
    #[doc = " assert_eq!(Ipv4Addr::new(80, 9, 12, 3).is_global(), true);"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ipv4", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn is_global(&self) -> bool {
        if u32::from_be_bytes(self.octets()) == 0xc0000009 ||
               u32::from_be_bytes(self.octets()) == 0xc000000a {
            return true;
        }
        !self.is_private() && !self.is_loopback() && !self.is_link_local() &&
            !self.is_broadcast() && !self.is_documentation() &&
            !self.is_shared() &&
            !(self.octets()[0] == 192 && self.octets()[1] == 0 &&
                  self.octets()[2] == 0) && !self.is_reserved() &&
            !self.is_benchmarking() && self.octets()[0] != 0
    }
    #[doc =
      " Returns [`true`] if this address is part of the Shared Address Space defined in"]
    #[doc = " [IETF RFC 6598] (`100.64.0.0/10`)."]
    #[doc = ""]
    #[doc = " [IETF RFC 6598]: https://tools.ietf.org/html/rfc6598"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc = " assert_eq!(Ipv4Addr::new(100, 64, 0, 0).is_shared(), true);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(100, 127, 255, 255).is_shared(), true);"]
    #[doc = " assert_eq!(Ipv4Addr::new(100, 128, 0, 0).is_shared(), false);"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ipv4", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn is_shared(&self) -> bool {
        self.octets()[0] == 100 &&
            (self.octets()[1] & 0b1100_0000 == 0b0100_0000)
    }
    #[doc =
      " Returns [`true`] if this address part of the `198.18.0.0/15` range, which is reserved for"]
    #[doc =
      " network devices benchmarking. This range is defined in [IETF RFC 2544] as `192.18.0.0`"]
    #[doc =
      " through `198.19.255.255` but [errata 423] corrects it to `198.18.0.0/15`."]
    #[doc = ""]
    #[doc = " [IETF RFC 2544]: https://tools.ietf.org/html/rfc2544"]
    #[doc = " [errata 423]: https://www.rfc-editor.org/errata/eid423"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv4Addr::new(198, 17, 255, 255).is_benchmarking(), false);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(198, 18, 0, 0).is_benchmarking(), true);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(198, 19, 255, 255).is_benchmarking(), true);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(198, 20, 0, 0).is_benchmarking(), false);"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ipv4", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn is_benchmarking(&self) -> bool {
        self.octets()[0] == 198 && (self.octets()[1] & 0xfe) == 18
    }
    #[doc =
      " Returns [`true`] if this address is reserved by IANA for future use. [IETF RFC 1112]"]
    #[doc =
      " defines the block of reserved addresses as `240.0.0.0/4`. This range normally includes the"]
    #[doc =
      " broadcast address `255.255.255.255`, but this implementation explicitly excludes it, since"]
    #[doc = " it is obviously not reserved for future use."]
    #[doc = ""]
    #[doc = " [IETF RFC 1112]: https://tools.ietf.org/html/rfc1112"]
    #[doc = ""]
    #[doc = " # Warning"]
    #[doc = ""]
    #[doc = " As IANA assigns new addresses, this method will be"]
    #[doc = " updated. This may result in non-reserved addresses being"]
    #[doc = " treated as reserved in code that relies on an outdated version"]
    #[doc = " of this method."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc = " assert_eq!(Ipv4Addr::new(240, 0, 0, 0).is_reserved(), true);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(255, 255, 255, 254).is_reserved(), true);"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv4Addr::new(239, 255, 255, 255).is_reserved(), false);"]
    #[doc =
      " // The broadcast address is not considered as reserved for future use by this implementation"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(255, 255, 255, 255).is_reserved(), false);"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ipv4", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn is_reserved(&self) -> bool {
        self.octets()[0] & 240 == 240 && !self.is_broadcast()
    }
    #[doc =
      " Returns [`true`] if this is a multicast address (`224.0.0.0/4`)."]
    #[doc = ""]
    #[doc =
      " Multicast addresses have a most significant octet between `224` and `239`,"]
    #[doc = " and is defined by [IETF RFC 5771]."]
    #[doc = ""]
    #[doc = " [IETF RFC 5771]: https://tools.ietf.org/html/rfc5771"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv4Addr::new(224, 254, 0, 0).is_multicast(), true);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(236, 168, 10, 65).is_multicast(), true);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(172, 16, 10, 65).is_multicast(), false);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv4", since = "1.50.0")]
    #[stable(since = "1.7.0", feature = "ip_17")]
    #[must_use]
    #[inline]
    pub const fn is_multicast(&self) -> bool {
        self.octets()[0] >= 224 && self.octets()[0] <= 239
    }
    #[doc =
      " Returns [`true`] if this is a broadcast address (`255.255.255.255`)."]
    #[doc = ""]
    #[doc =
      " A broadcast address has all octets set to `255` as defined in [IETF RFC 919]."]
    #[doc = ""]
    #[doc = " [IETF RFC 919]: https://tools.ietf.org/html/rfc919"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv4Addr::new(255, 255, 255, 255).is_broadcast(), true);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(236, 168, 10, 65).is_broadcast(), false);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv4", since = "1.50.0")]
    #[stable(since = "1.7.0", feature = "ip_17")]
    #[must_use]
    #[inline]
    pub const fn is_broadcast(&self) -> bool {
        u32::from_be_bytes(self.octets()) ==
            u32::from_be_bytes(Self::BROADCAST.octets())
    }
    #[doc =
      " Returns [`true`] if this address is in a range designated for documentation."]
    #[doc = ""]
    #[doc = " This is defined in [IETF RFC 5737]:"]
    #[doc = ""]
    #[doc = " - `192.0.2.0/24` (TEST-NET-1)"]
    #[doc = " - `198.51.100.0/24` (TEST-NET-2)"]
    #[doc = " - `203.0.113.0/24` (TEST-NET-3)"]
    #[doc = ""]
    #[doc = " [IETF RFC 5737]: https://tools.ietf.org/html/rfc5737"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv4Addr::new(192, 0, 2, 255).is_documentation(), true);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(198, 51, 100, 65).is_documentation(), true);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(203, 0, 113, 6).is_documentation(), true);"]
    #[doc =
      " assert_eq!(Ipv4Addr::new(193, 34, 17, 19).is_documentation(), false);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv4", since = "1.50.0")]
    #[stable(since = "1.7.0", feature = "ip_17")]
    #[must_use]
    #[inline]
    pub const fn is_documentation(&self) -> bool {
        matches!(self.octets(), [192, 0, 2, _] | [198, 51, 100, _] |
                 [203, 0, 113, _])
    }
    #[doc =
      " Converts this address to an [IPv4-compatible] [`IPv6` address]."]
    #[doc = ""]
    #[doc = " `a.b.c.d` becomes `::a.b.c.d`"]
    #[doc = ""]
    #[doc =
      " Note that IPv4-compatible addresses have been officially deprecated."]
    #[doc =
      " If you don't explicitly need an IPv4-compatible address for legacy reasons, consider using `to_ipv6_mapped` instead."]
    #[doc = ""]
    #[doc = " [IPv4-compatible]: Ipv6Addr#ipv4-compatible-ipv6-addresses"]
    #[doc = " [`IPv6` address]: Ipv6Addr"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::{Ipv4Addr, Ipv6Addr};"]
    #[doc = ""]
    #[doc = " assert_eq!("]
    #[doc = "     Ipv4Addr::new(192, 0, 2, 255).to_ipv6_compatible(),"]
    #[doc = "     Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0xc000, 0x2ff)"]
    #[doc = " );"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv4", since = "1.50.0")]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use =
      "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub const fn to_ipv6_compatible(&self) -> Ipv6Addr {
        let [a, b, c, d] = self.octets();
        Ipv6Addr{inner:
                     c::in6_addr{s6_addr:
                                     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, a,
                                      b, c, d],},}
    }
    #[doc = " Converts this address to an [IPv4-mapped] [`IPv6` address]."]
    #[doc = ""]
    #[doc = " `a.b.c.d` becomes `::ffff:a.b.c.d`"]
    #[doc = ""]
    #[doc = " [IPv4-mapped]: Ipv6Addr#ipv4-mapped-ipv6-addresses"]
    #[doc = " [`IPv6` address]: Ipv6Addr"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::{Ipv4Addr, Ipv6Addr};"]
    #[doc = ""]
    #[doc = " assert_eq!(Ipv4Addr::new(192, 0, 2, 255).to_ipv6_mapped(),"]
    #[doc =
      "            Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc000, 0x2ff));"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv4", since = "1.50.0")]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use =
      "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub const fn to_ipv6_mapped(&self) -> Ipv6Addr {
        let [a, b, c, d] = self.octets();
        Ipv6Addr{inner:
                     c::in6_addr{s6_addr:
                                     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF,
                                      0xFF, a, b, c, d],},}
    }
}
#[stable(feature = "ip_addr", since = "1.7.0")]
impl fmt::Display for IpAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpAddr::V4(ip) => ip.fmt(fmt),
            IpAddr::V6(ip) => ip.fmt(fmt),
        }
    }
}
#[stable(feature = "ip_addr", since = "1.7.0")]
impl fmt::Debug for IpAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}
#[stable(feature = "ip_from_ip", since = "1.16.0")]
impl From<Ipv4Addr> for IpAddr {
    #[doc = " Copies this address to a new `IpAddr::V4`."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::{IpAddr, Ipv4Addr};"]
    #[doc = ""]
    #[doc = " let addr = Ipv4Addr::new(127, 0, 0, 1);"]
    #[doc = ""]
    #[doc = " assert_eq!("]
    #[doc = "     IpAddr::V4(addr),"]
    #[doc = "     IpAddr::from(addr)"]
    #[doc = " )"]
    #[doc = " ```"]
    #[inline]
    fn from(ipv4: Ipv4Addr) -> IpAddr { IpAddr::V4(ipv4) }
}
#[stable(feature = "ip_from_ip", since = "1.16.0")]
impl From<Ipv6Addr> for IpAddr {
    #[doc = " Copies this address to a new `IpAddr::V6`."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::{IpAddr, Ipv6Addr};"]
    #[doc = ""]
    #[doc =
      " let addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff);"]
    #[doc = ""]
    #[doc = " assert_eq!("]
    #[doc = "     IpAddr::V6(addr),"]
    #[doc = "     IpAddr::from(addr)"]
    #[doc = " );"]
    #[doc = " ```"]
    #[inline]
    fn from(ipv6: Ipv6Addr) -> IpAddr { IpAddr::V6(ipv6) }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Display for Ipv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octets = self.octets();
        if fmt.precision().is_none() && fmt.width().is_none() {
            write!(fmt, "{}.{}.{}.{}", octets [0], octets [1], octets [2],
                   octets [3])
        } else {
            const IPV4_BUF_LEN: usize = 15;
            let mut buf = [0u8; IPV4_BUF_LEN];
            let mut buf_slice = &mut buf[..];
            write!(buf_slice, "{}.{}.{}.{}", octets [0], octets [1], octets
                   [2], octets [3]).unwrap();
            let len = IPV4_BUF_LEN - buf_slice.len();
            let buf = unsafe { crate::str::from_utf8_unchecked(&buf[..len]) };
            fmt.pad(buf)
        }
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Debug for Ipv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl Clone for Ipv4Addr {
    #[inline]
    fn clone(&self) -> Ipv4Addr { *self }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl PartialEq for Ipv4Addr {
    #[inline]
    fn eq(&self, other: &Ipv4Addr) -> bool {
        self.inner.s_addr == other.inner.s_addr
    }
}
#[stable(feature = "ip_cmp", since = "1.16.0")]
impl PartialEq<Ipv4Addr> for IpAddr {
    #[inline]
    fn eq(&self, other: &Ipv4Addr) -> bool {
        match self { IpAddr::V4(v4) => v4 == other, IpAddr::V6(_) => false, }
    }
}
#[stable(feature = "ip_cmp", since = "1.16.0")]
impl PartialEq<IpAddr> for Ipv4Addr {
    #[inline]
    fn eq(&self, other: &IpAddr) -> bool {
        match other { IpAddr::V4(v4) => self == v4, IpAddr::V6(_) => false, }
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl Eq for Ipv4Addr { }
#[stable(feature = "rust1", since = "1.0.0")]
impl hash::Hash for Ipv4Addr {
    #[inline]
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        { self.inner.s_addr }.hash(s)
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl PartialOrd for Ipv4Addr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv4Addr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[stable(feature = "ip_cmp", since = "1.16.0")]
impl PartialOrd<Ipv4Addr> for IpAddr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv4Addr) -> Option<Ordering> {
        match self {
            IpAddr::V4(v4) => v4.partial_cmp(other),
            IpAddr::V6(_) => Some(Ordering::Greater),
        }
    }
}
#[stable(feature = "ip_cmp", since = "1.16.0")]
impl PartialOrd<IpAddr> for Ipv4Addr {
    #[inline]
    fn partial_cmp(&self, other: &IpAddr) -> Option<Ordering> {
        match other {
            IpAddr::V4(v4) => self.partial_cmp(v4),
            IpAddr::V6(_) => Some(Ordering::Less),
        }
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl Ord for Ipv4Addr {
    #[inline]
    fn cmp(&self, other: &Ipv4Addr) -> Ordering {
        u32::from_be(self.inner.s_addr).cmp(&u32::from_be(other.inner.s_addr))
    }
}
impl IntoInner<c::in_addr> for Ipv4Addr {
    #[inline]
    fn into_inner(self) -> c::in_addr { self.inner }
}
#[stable(feature = "ip_u32", since = "1.1.0")]
impl From<Ipv4Addr> for u32 {
    #[doc = " Converts an `Ipv4Addr` into a host byte order `u32`."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc = " let addr = Ipv4Addr::new(0x12, 0x34, 0x56, 0x78);"]
    #[doc = " assert_eq!(0x12345678, u32::from(addr));"]
    #[doc = " ```"]
    #[inline]
    fn from(ip: Ipv4Addr) -> u32 {
        let ip = ip.octets();
        u32::from_be_bytes(ip)
    }
}
#[stable(feature = "ip_u32", since = "1.1.0")]
impl From<u32> for Ipv4Addr {
    #[doc = " Converts a host byte order `u32` into an `Ipv4Addr`."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc = " let addr = Ipv4Addr::from(0x12345678);"]
    #[doc = " assert_eq!(Ipv4Addr::new(0x12, 0x34, 0x56, 0x78), addr);"]
    #[doc = " ```"]
    #[inline]
    fn from(ip: u32) -> Ipv4Addr { Ipv4Addr::from(ip.to_be_bytes()) }
}
#[stable(feature = "from_slice_v4", since = "1.9.0")]
impl From<[u8; 4]> for Ipv4Addr {
    #[doc = " Creates an `Ipv4Addr` from a four element byte array."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv4Addr;"]
    #[doc = ""]
    #[doc = " let addr = Ipv4Addr::from([13u8, 12u8, 11u8, 10u8]);"]
    #[doc = " assert_eq!(Ipv4Addr::new(13, 12, 11, 10), addr);"]
    #[doc = " ```"]
    #[inline]
    fn from(octets: [u8; 4]) -> Ipv4Addr {
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])
    }
}
#[stable(feature = "ip_from_slice", since = "1.17.0")]
impl From<[u8; 4]> for IpAddr {
    #[doc = " Creates an `IpAddr::V4` from a four element byte array."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::{IpAddr, Ipv4Addr};"]
    #[doc = ""]
    #[doc = " let addr = IpAddr::from([13u8, 12u8, 11u8, 10u8]);"]
    #[doc = " assert_eq!(IpAddr::V4(Ipv4Addr::new(13, 12, 11, 10)), addr);"]
    #[doc = " ```"]
    #[inline]
    fn from(octets: [u8; 4]) -> IpAddr { IpAddr::V4(Ipv4Addr::from(octets)) }
}
impl Ipv6Addr {
    #[doc = " Creates a new IPv6 address from eight 16-bit segments."]
    #[doc = ""]
    #[doc = " The result will represent the IP address `a:b:c:d:e:f:g:h`."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc =
      " let addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv6", since = "1.32.0")]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use]
    #[inline]
    pub const fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16,
                     h: u16) -> Ipv6Addr {
        let addr16 =
            [a.to_be(), b.to_be(), c.to_be(), d.to_be(), e.to_be(), f.to_be(),
             g.to_be(), h.to_be()];
        Ipv6Addr{inner:
                     c::in6_addr{s6_addr:
                                     unsafe {
                                         transmute::<_, [u8; 16]>(addr16)
                                     },},}
    }
    #[doc = " An IPv6 address representing localhost: `::1`."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc = " let addr = Ipv6Addr::LOCALHOST;"]
    #[doc = " assert_eq!(addr, Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));"]
    #[doc = " ```"]
    #[stable(feature = "ip_constructors", since = "1.30.0")]
    pub const LOCALHOST: Self = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    #[doc = " An IPv6 address representing the unspecified address: `::`"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc = " let addr = Ipv6Addr::UNSPECIFIED;"]
    #[doc = " assert_eq!(addr, Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0));"]
    #[doc = " ```"]
    #[stable(feature = "ip_constructors", since = "1.30.0")]
    pub const UNSPECIFIED: Self = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
    #[doc = " Returns the eight 16-bit segments that make up this address."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).segments(),"]
    #[doc = "            [0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff]);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv6", since = "1.50.0")]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use]
    #[inline]
    pub const fn segments(&self) -> [u16; 8] {
        let [a, b, c, d, e, f, g, h] =
            unsafe { transmute::<_, [u16; 8]>(self.inner.s6_addr) };
        [u16::from_be(a), u16::from_be(b), u16::from_be(c), u16::from_be(d),
         u16::from_be(e), u16::from_be(f), u16::from_be(g), u16::from_be(h)]
    }
    #[doc = " Returns [`true`] for the special 'unspecified' address (`::`)."]
    #[doc = ""]
    #[doc = " This property is defined in [IETF RFC 4291]."]
    #[doc = ""]
    #[doc = " [IETF RFC 4291]: https://tools.ietf.org/html/rfc4291"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_unspecified(), false);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).is_unspecified(), true);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv6", since = "1.50.0")]
    #[stable(since = "1.7.0", feature = "ip_17")]
    #[must_use]
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        u128::from_be_bytes(self.octets()) ==
            u128::from_be_bytes(Ipv6Addr::UNSPECIFIED.octets())
    }
    #[doc = " Returns [`true`] if this is the [loopback address] (`::1`),"]
    #[doc = " as defined in [IETF RFC 4291 section 2.5.3]."]
    #[doc = ""]
    #[doc = " Contrary to IPv4, in IPv6 there is only one loopback address."]
    #[doc = ""]
    #[doc = " [loopback address]: Ipv6Addr::LOCALHOST"]
    #[doc =
      " [IETF RFC 4291 section 2.5.3]: https://tools.ietf.org/html/rfc4291#section-2.5.3"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_loopback(), false);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0x1).is_loopback(), true);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv6", since = "1.50.0")]
    #[stable(since = "1.7.0", feature = "ip_17")]
    #[must_use]
    #[inline]
    pub const fn is_loopback(&self) -> bool {
        u128::from_be_bytes(self.octets()) ==
            u128::from_be_bytes(Ipv6Addr::LOCALHOST.octets())
    }
    #[doc =
      " Returns [`true`] if the address appears to be globally routable."]
    #[doc = ""]
    #[doc = " The following return [`false`]:"]
    #[doc = ""]
    #[doc = " - the loopback address"]
    #[doc = " - link-local and unique local unicast addresses"]
    #[doc =
      " - interface-, link-, realm-, admin- and site-local multicast addresses"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = ""]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_global(), true);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0x1).is_global(), false);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0x1c9, 0, 0, 0xafc8, 0, 0x1).is_global(), true);"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ipv6", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn is_global(&self) -> bool {
        match self.multicast_scope() {
            Some(Ipv6MulticastScope::Global) => true,
            None => self.is_unicast_global(),
            _ => false,
        }
    }
    #[doc =
      " Returns [`true`] if this is a unique local address (`fc00::/7`)."]
    #[doc = ""]
    #[doc = " This property is defined in [IETF RFC 4193]."]
    #[doc = ""]
    #[doc = " [IETF RFC 4193]: https://tools.ietf.org/html/rfc4193"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = ""]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_unique_local(), false);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0xfc02, 0, 0, 0, 0, 0, 0, 0).is_unique_local(), true);"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ipv6", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn is_unique_local(&self) -> bool {
        (self.segments()[0] & 0xfe00) == 0xfc00
    }
    #[doc =
      " Returns [`true`] if this is a unicast address, as defined by [IETF RFC 4291]."]
    #[doc =
      " Any address that is not a [multicast address] (`ff00::/8`) is unicast."]
    #[doc = ""]
    #[doc = " [IETF RFC 4291]: https://tools.ietf.org/html/rfc4291"]
    #[doc = " [multicast address]: Ipv6Addr::is_multicast"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = ""]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc = " // The unspecified and loopback addresses are unicast."]
    #[doc = " assert_eq!(Ipv6Addr::UNSPECIFIED.is_unicast(), true);"]
    #[doc = " assert_eq!(Ipv6Addr::LOCALHOST.is_unicast(), true);"]
    #[doc = ""]
    #[doc =
      " // Any address that is not a multicast address (`ff00::/8`) is unicast."]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0).is_unicast(), true);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0).is_unicast(), false);"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ipv6", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn is_unicast(&self) -> bool { !self.is_multicast() }
    #[doc =
      " Returns `true` if the address is a unicast address with link-local scope,"]
    #[doc = " as defined in [RFC 4291]."]
    #[doc = ""]
    #[doc =
      " A unicast address has link-local scope if it has the prefix `fe80::/10`, as per [RFC 4291 section 2.4]."]
    #[doc =
      " Note that this encompasses more addresses than those defined in [RFC 4291 section 2.5.6],"]
    #[doc =
      " which describes \"Link-Local IPv6 Unicast Addresses\" as having the following stricter format:"]
    #[doc = ""]
    #[doc = " ```text"]
    #[doc =
      " | 10 bits  |         54 bits         |          64 bits           |"]
    #[doc =
      " +----------+-------------------------+----------------------------+"]
    #[doc =
      " |1111111010|           0             |       interface ID         |"]
    #[doc =
      " +----------+-------------------------+----------------------------+"]
    #[doc = " ```"]
    #[doc =
      " So while currently the only addresses with link-local scope an application will encounter are all in `fe80::/64`,"]
    #[doc =
      " this might change in the future with the publication of new standards. More addresses in `fe80::/10` could be allocated,"]
    #[doc = " and those addresses will have link-local scope."]
    #[doc = ""]
    #[doc =
      " Also note that while [RFC 4291 section 2.5.3] mentions about the [loopback address] (`::1`) that \"it is treated as having Link-Local scope\","]
    #[doc =
      " this does not mean that the loopback address actually has link-local scope and this method will return `false` on it."]
    #[doc = ""]
    #[doc = " [RFC 4291]: https://tools.ietf.org/html/rfc4291"]
    #[doc =
      " [RFC 4291 section 2.4]: https://tools.ietf.org/html/rfc4291#section-2.4"]
    #[doc =
      " [RFC 4291 section 2.5.3]: https://tools.ietf.org/html/rfc4291#section-2.5.3"]
    #[doc =
      " [RFC 4291 section 2.5.6]: https://tools.ietf.org/html/rfc4291#section-2.5.6"]
    #[doc = " [loopback address]: Ipv6Addr::LOCALHOST"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = ""]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc =
      " // The loopback address (`::1`) does not actually have link-local scope."]
    #[doc =
      " assert_eq!(Ipv6Addr::LOCALHOST.is_unicast_link_local(), false);"]
    #[doc = ""]
    #[doc = " // Only addresses in `fe80::/10` have link-local scope."]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0).is_unicast_link_local(), false);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 0).is_unicast_link_local(), true);"]
    #[doc = ""]
    #[doc =
      " // Addresses outside the stricter `fe80::/64` also have link-local scope."]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0xfe80, 0, 0, 1, 0, 0, 0, 0).is_unicast_link_local(), true);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0xfe81, 0, 0, 0, 0, 0, 0, 0).is_unicast_link_local(), true);"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ipv6", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn is_unicast_link_local(&self) -> bool {
        (self.segments()[0] & 0xffc0) == 0xfe80
    }
    #[doc =
      " Returns [`true`] if this is an address reserved for documentation"]
    #[doc = " (`2001:db8::/32`)."]
    #[doc = ""]
    #[doc = " This property is defined in [IETF RFC 3849]."]
    #[doc = ""]
    #[doc = " [IETF RFC 3849]: https://tools.ietf.org/html/rfc3849"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = ""]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_documentation(), false);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0).is_documentation(), true);"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ipv6", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn is_documentation(&self) -> bool {
        (self.segments()[0] == 0x2001) && (self.segments()[1] == 0xdb8)
    }
    #[doc =
      " Returns [`true`] if this is an address reserved for benchmarking (`2001:2::/48`)."]
    #[doc = ""]
    #[doc =
      " This property is defined in [IETF RFC 5180], where it is mistakenly specified as covering the range `2001:0200::/48`."]
    #[doc =
      " This is corrected in [IETF RFC Errata 1752] to `2001:0002::/48`."]
    #[doc = ""]
    #[doc = " [IETF RFC 5180]: https://tools.ietf.org/html/rfc5180"]
    #[doc =
      " [IETF RFC Errata 1752]: https://www.rfc-editor.org/errata_search.php?eid=1752"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = ""]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc613, 0x0).is_benchmarking(), false);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0x2001, 0x2, 0, 0, 0, 0, 0, 0).is_benchmarking(), true);"]
    #[doc = " ```"]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn is_benchmarking(&self) -> bool {
        (self.segments()[0] == 0x2001) && (self.segments()[1] == 0x2) &&
            (self.segments()[2] == 0)
    }
    #[doc =
      " Returns [`true`] if the address is a globally routable unicast address."]
    #[doc = ""]
    #[doc = " The following return false:"]
    #[doc = ""]
    #[doc = " - the loopback address"]
    #[doc = " - the link-local addresses"]
    #[doc = " - unique local addresses"]
    #[doc = " - the unspecified address"]
    #[doc = " - the address range reserved for documentation"]
    #[doc = ""]
    #[doc =
      " This method returns [`true`] for site-local addresses as per [RFC 4291 section 2.5.7]"]
    #[doc = ""]
    #[doc = " ```no_rust"]
    #[doc =
      " The special behavior of [the site-local unicast] prefix defined in [RFC3513] must no longer"]
    #[doc =
      " be supported in new implementations (i.e., new implementations must treat this prefix as"]
    #[doc = " Global Unicast)."]
    #[doc = " ```"]
    #[doc = ""]
    #[doc =
      " [RFC 4291 section 2.5.7]: https://tools.ietf.org/html/rfc4291#section-2.5.7"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = ""]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0).is_unicast_global(), false);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_unicast_global(), true);"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ipv6", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn is_unicast_global(&self) -> bool {
        self.is_unicast() && !self.is_loopback() &&
            !self.is_unicast_link_local() && !self.is_unique_local() &&
            !self.is_unspecified() && !self.is_documentation()
    }
    #[doc =
      " Returns the address's multicast scope if the address is multicast."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = ""]
    #[doc = " use std::net::{Ipv6Addr, Ipv6MulticastScope};"]
    #[doc = ""]
    #[doc = " assert_eq!("]
    #[doc =
      "     Ipv6Addr::new(0xff0e, 0, 0, 0, 0, 0, 0, 0).multicast_scope(),"]
    #[doc = "     Some(Ipv6MulticastScope::Global)"]
    #[doc = " );"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).multicast_scope(), None);"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ipv6", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use]
    #[inline]
    pub const fn multicast_scope(&self) -> Option<Ipv6MulticastScope> {
        if self.is_multicast() {
            match self.segments()[0] & 0x000f {
                1 => Some(Ipv6MulticastScope::InterfaceLocal),
                2 => Some(Ipv6MulticastScope::LinkLocal),
                3 => Some(Ipv6MulticastScope::RealmLocal),
                4 => Some(Ipv6MulticastScope::AdminLocal),
                5 => Some(Ipv6MulticastScope::SiteLocal),
                8 => Some(Ipv6MulticastScope::OrganizationLocal),
                14 => Some(Ipv6MulticastScope::Global),
                _ => None,
            }
        } else { None }
    }
    #[doc = " Returns [`true`] if this is a multicast address (`ff00::/8`)."]
    #[doc = ""]
    #[doc = " This property is defined by [IETF RFC 4291]."]
    #[doc = ""]
    #[doc = " [IETF RFC 4291]: https://tools.ietf.org/html/rfc4291"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0).is_multicast(), true);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_multicast(), false);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv6", since = "1.50.0")]
    #[stable(since = "1.7.0", feature = "ip_17")]
    #[must_use]
    #[inline]
    pub const fn is_multicast(&self) -> bool {
        (self.segments()[0] & 0xff00) == 0xff00
    }
    #[doc =
      " Converts this address to an [`IPv4` address] if it's an [IPv4-mapped] address,"]
    #[doc =
      " as defined in [IETF RFC 4291 section 2.5.5.2], otherwise returns [`None`]."]
    #[doc = ""]
    #[doc = " `::ffff:a.b.c.d` becomes `a.b.c.d`."]
    #[doc = " All addresses *not* starting with `::ffff` will return `None`."]
    #[doc = ""]
    #[doc = " [`IPv4` address]: Ipv4Addr"]
    #[doc = " [IPv4-mapped]: Ipv6Addr"]
    #[doc =
      " [IETF RFC 4291 section 2.5.5.2]: https://tools.ietf.org/html/rfc4291#section-2.5.5.2"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = ""]
    #[doc = " use std::net::{Ipv4Addr, Ipv6Addr};"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0).to_ipv4_mapped(), None);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).to_ipv4_mapped(),"]
    #[doc = "            Some(Ipv4Addr::new(192, 10, 2, 255)));"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1).to_ipv4_mapped(), None);"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ipv6", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use =
      "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub const fn to_ipv4_mapped(&self) -> Option<Ipv4Addr> {
        match self.octets() {
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, a, b, c, d] => {
                Some(Ipv4Addr::new(a, b, c, d))
            }
            _ => None,
        }
    }
    #[doc = " Converts this address to an [`IPv4` address] if it is either"]
    #[doc =
      " an [IPv4-compatible] address as defined in [IETF RFC 4291 section 2.5.5.1],"]
    #[doc =
      " or an [IPv4-mapped] address as defined in [IETF RFC 4291 section 2.5.5.2],"]
    #[doc = " otherwise returns [`None`]."]
    #[doc = ""]
    #[doc = " `::a.b.c.d` and `::ffff:a.b.c.d` become `a.b.c.d`"]
    #[doc =
      " All addresses *not* starting with either all zeroes or `::ffff` will return `None`."]
    #[doc = ""]
    #[doc = " [`IPv4` address]: Ipv4Addr"]
    #[doc = " [IPv4-compatible]: Ipv6Addr#ipv4-compatible-ipv6-addresses"]
    #[doc = " [IPv4-mapped]: Ipv6Addr#ipv4-mapped-ipv6-addresses"]
    #[doc =
      " [IETF RFC 4291 section 2.5.5.1]: https://tools.ietf.org/html/rfc4291#section-2.5.5.1"]
    #[doc =
      " [IETF RFC 4291 section 2.5.5.2]: https://tools.ietf.org/html/rfc4291#section-2.5.5.2"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::{Ipv4Addr, Ipv6Addr};"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0).to_ipv4(), None);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).to_ipv4(),"]
    #[doc = "            Some(Ipv4Addr::new(192, 10, 2, 255)));"]
    #[doc = " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1).to_ipv4(),"]
    #[doc = "            Some(Ipv4Addr::new(0, 0, 0, 1)));"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv6", since = "1.50.0")]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use =
      "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub const fn to_ipv4(&self) -> Option<Ipv4Addr> {
        if let [0, 0, 0, 0, 0, 0 | 0xffff, ab, cd] = self.segments() {
            let [a, b] = ab.to_be_bytes();
            let [c, d] = cd.to_be_bytes();
            Some(Ipv4Addr::new(a, b, c, d))
        } else { None }
    }
    #[doc =
      " Converts this address to an `IpAddr::V4` if it is an IPv4-mapped addresses, otherwise it"]
    #[doc = " returns self wrapped in an `IpAddr::V6`."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " #![feature(ip)]"]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0x7f00, 0x1).is_loopback(), false);"]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0x7f00, 0x1).to_canonical().is_loopback(), true);"]
    #[doc = " ```"]
    #[rustc_const_unstable(feature = "const_ipv6", issue = "76205")]
    #[unstable(feature = "ip", issue = "27709")]
    #[must_use =
      "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub const fn to_canonical(&self) -> IpAddr {
        if let Some(mapped) = self.to_ipv4_mapped() {
            return IpAddr::V4(mapped);
        }
        IpAddr::V6(*self)
    }
    #[doc =
      " Returns the sixteen eight-bit integers the IPv6 address consists of."]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc =
      " assert_eq!(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0).octets(),"]
    #[doc =
      "            [255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);"]
    #[doc = " ```"]
    #[rustc_const_stable(feature = "const_ipv6", since = "1.32.0")]
    #[stable(feature = "ipv6_to_octets", since = "1.12.0")]
    #[must_use]
    #[inline]
    pub const fn octets(&self) -> [u8; 16] { self.inner.s6_addr }
}
#[doc = " Write an Ipv6Addr, conforming to the canonical style described by"]
#[doc = " [RFC 5952](https://tools.ietf.org/html/rfc5952)."]
#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Display for Ipv6Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.precision().is_none() && f.width().is_none() {
            let segments = self.segments();
            if self.is_unspecified() {
                f.write_str("::")
            } else if self.is_loopback() {
                f.write_str("::1")
            } else if let Some(ipv4) = self.to_ipv4() {
                match segments[5] {
                    0 => write!(f, "::{}", ipv4),
                    0xffff => write!(f, "::ffff:{}", ipv4),
                    _ => unreachable!(),
                }
            } else {
                #[derive(Copy, Clone, Default)]
                struct Span {
                    start: usize,
                    len: usize,
                }
                let zeroes =
                    {
                        let mut longest = Span::default();
                        let mut current = Span::default();
                        for (i, &segment) in segments.iter().enumerate() {
                            if segment == 0 {
                                if current.len == 0 { current.start = i; }
                                current.len += 1;
                                if current.len > longest.len {
                                    longest = current;
                                }
                            } else { current = Span::default(); }
                        }
                        longest
                    };
                #[doc = " Write a colon-separated part of the address"]
                #[inline]
                fn fmt_subslice(f: &mut fmt::Formatter<'_>, chunk: &[u16])
                 -> fmt::Result {
                    if let Some((first, tail)) = chunk.split_first() {
                        write!(f, "{:x}", first)?;
                        for segment in tail {
                            f.write_char(':')?;
                            write!(f, "{:x}", segment)?;
                        }
                    }
                    Ok(())
                }
                if zeroes.len > 1 {
                    fmt_subslice(f, &segments[..zeroes.start])?;
                    f.write_str("::")?;
                    fmt_subslice(f, &segments[zeroes.start + zeroes.len..])
                } else { fmt_subslice(f, &segments) }
            }
        } else {
            const IPV6_BUF_LEN: usize = (4 * 8) + 7;
            let mut buf = [0u8; IPV6_BUF_LEN];
            let mut buf_slice = &mut buf[..];
            write!(buf_slice, "{}", self).unwrap();
            let len = IPV6_BUF_LEN - buf_slice.len();
            let buf = unsafe { crate::str::from_utf8_unchecked(&buf[..len]) };
            f.pad(buf)
        }
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Debug for Ipv6Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl Clone for Ipv6Addr {
    #[inline]
    fn clone(&self) -> Ipv6Addr { *self }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl PartialEq for Ipv6Addr {
    #[inline]
    fn eq(&self, other: &Ipv6Addr) -> bool {
        self.inner.s6_addr == other.inner.s6_addr
    }
}
#[stable(feature = "ip_cmp", since = "1.16.0")]
impl PartialEq<IpAddr> for Ipv6Addr {
    #[inline]
    fn eq(&self, other: &IpAddr) -> bool {
        match other { IpAddr::V4(_) => false, IpAddr::V6(v6) => self == v6, }
    }
}
#[stable(feature = "ip_cmp", since = "1.16.0")]
impl PartialEq<Ipv6Addr> for IpAddr {
    #[inline]
    fn eq(&self, other: &Ipv6Addr) -> bool {
        match self { IpAddr::V4(_) => false, IpAddr::V6(v6) => v6 == other, }
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl Eq for Ipv6Addr { }
#[stable(feature = "rust1", since = "1.0.0")]
impl hash::Hash for Ipv6Addr {
    #[inline]
    fn hash<H: hash::Hasher>(&self, s: &mut H) { self.inner.s6_addr.hash(s) }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl PartialOrd for Ipv6Addr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv6Addr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[stable(feature = "ip_cmp", since = "1.16.0")]
impl PartialOrd<Ipv6Addr> for IpAddr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv6Addr) -> Option<Ordering> {
        match self {
            IpAddr::V4(_) => Some(Ordering::Less),
            IpAddr::V6(v6) => v6.partial_cmp(other),
        }
    }
}
#[stable(feature = "ip_cmp", since = "1.16.0")]
impl PartialOrd<IpAddr> for Ipv6Addr {
    #[inline]
    fn partial_cmp(&self, other: &IpAddr) -> Option<Ordering> {
        match other {
            IpAddr::V4(_) => Some(Ordering::Greater),
            IpAddr::V6(v6) => self.partial_cmp(v6),
        }
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl Ord for Ipv6Addr {
    #[inline]
    fn cmp(&self, other: &Ipv6Addr) -> Ordering {
        self.segments().cmp(&other.segments())
    }
}
impl AsInner<c::in6_addr> for Ipv6Addr {
    #[inline]
    fn as_inner(&self) -> &c::in6_addr { &self.inner }
}
impl FromInner<c::in6_addr> for Ipv6Addr {
    #[inline]
    fn from_inner(addr: c::in6_addr) -> Ipv6Addr { Ipv6Addr{inner: addr,} }
}
#[stable(feature = "i128", since = "1.26.0")]
impl From<Ipv6Addr> for u128 {
    #[doc = " Convert an `Ipv6Addr` into a host byte order `u128`."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc = " let addr = Ipv6Addr::new("]
    #[doc = "     0x1020, 0x3040, 0x5060, 0x7080,"]
    #[doc = "     0x90A0, 0xB0C0, 0xD0E0, 0xF00D,"]
    #[doc = " );"]
    #[doc =
      " assert_eq!(0x102030405060708090A0B0C0D0E0F00D_u128, u128::from(addr));"]
    #[doc = " ```"]
    #[inline]
    fn from(ip: Ipv6Addr) -> u128 {
        let ip = ip.octets();
        u128::from_be_bytes(ip)
    }
}
#[stable(feature = "i128", since = "1.26.0")]
impl From<u128> for Ipv6Addr {
    #[doc = " Convert a host byte order `u128` into an `Ipv6Addr`."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc =
      " let addr = Ipv6Addr::from(0x102030405060708090A0B0C0D0E0F00D_u128);"]
    #[doc = " assert_eq!("]
    #[doc = "     Ipv6Addr::new("]
    #[doc = "         0x1020, 0x3040, 0x5060, 0x7080,"]
    #[doc = "         0x90A0, 0xB0C0, 0xD0E0, 0xF00D,"]
    #[doc = "     ),"]
    #[doc = "     addr);"]
    #[doc = " ```"]
    #[inline]
    fn from(ip: u128) -> Ipv6Addr { Ipv6Addr::from(ip.to_be_bytes()) }
}
#[stable(feature = "ipv6_from_octets", since = "1.9.0")]
impl From<[u8; 16]> for Ipv6Addr {
    #[doc = " Creates an `Ipv6Addr` from a sixteen element byte array."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc = " let addr = Ipv6Addr::from(["]
    #[doc = "     25u8, 24u8, 23u8, 22u8, 21u8, 20u8, 19u8, 18u8,"]
    #[doc = "     17u8, 16u8, 15u8, 14u8, 13u8, 12u8, 11u8, 10u8,"]
    #[doc = " ]);"]
    #[doc = " assert_eq!("]
    #[doc = "     Ipv6Addr::new("]
    #[doc = "         0x1918, 0x1716,"]
    #[doc = "         0x1514, 0x1312,"]
    #[doc = "         0x1110, 0x0f0e,"]
    #[doc = "         0x0d0c, 0x0b0a"]
    #[doc = "     ),"]
    #[doc = "     addr"]
    #[doc = " );"]
    #[doc = " ```"]
    #[inline]
    fn from(octets: [u8; 16]) -> Ipv6Addr {
        let inner = c::in6_addr{s6_addr: octets,};
        Ipv6Addr::from_inner(inner)
    }
}
#[stable(feature = "ipv6_from_segments", since = "1.16.0")]
impl From<[u16; 8]> for Ipv6Addr {
    #[doc = " Creates an `Ipv6Addr` from an eight element 16-bit array."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::Ipv6Addr;"]
    #[doc = ""]
    #[doc = " let addr = Ipv6Addr::from(["]
    #[doc = "     525u16, 524u16, 523u16, 522u16,"]
    #[doc = "     521u16, 520u16, 519u16, 518u16,"]
    #[doc = " ]);"]
    #[doc = " assert_eq!("]
    #[doc = "     Ipv6Addr::new("]
    #[doc = "         0x20d, 0x20c,"]
    #[doc = "         0x20b, 0x20a,"]
    #[doc = "         0x209, 0x208,"]
    #[doc = "         0x207, 0x206"]
    #[doc = "     ),"]
    #[doc = "     addr"]
    #[doc = " );"]
    #[doc = " ```"]
    #[inline]
    fn from(segments: [u16; 8]) -> Ipv6Addr {
        let [a, b, c, d, e, f, g, h] = segments;
        Ipv6Addr::new(a, b, c, d, e, f, g, h)
    }
}
#[stable(feature = "ip_from_slice", since = "1.17.0")]
impl From<[u8; 16]> for IpAddr {
    #[doc = " Creates an `IpAddr::V6` from a sixteen element byte array."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::{IpAddr, Ipv6Addr};"]
    #[doc = ""]
    #[doc = " let addr = IpAddr::from(["]
    #[doc = "     25u8, 24u8, 23u8, 22u8, 21u8, 20u8, 19u8, 18u8,"]
    #[doc = "     17u8, 16u8, 15u8, 14u8, 13u8, 12u8, 11u8, 10u8,"]
    #[doc = " ]);"]
    #[doc = " assert_eq!("]
    #[doc = "     IpAddr::V6(Ipv6Addr::new("]
    #[doc = "         0x1918, 0x1716,"]
    #[doc = "         0x1514, 0x1312,"]
    #[doc = "         0x1110, 0x0f0e,"]
    #[doc = "         0x0d0c, 0x0b0a"]
    #[doc = "     )),"]
    #[doc = "     addr"]
    #[doc = " );"]
    #[doc = " ```"]
    #[inline]
    fn from(octets: [u8; 16]) -> IpAddr { IpAddr::V6(Ipv6Addr::from(octets)) }
}
#[stable(feature = "ip_from_slice", since = "1.17.0")]
impl From<[u16; 8]> for IpAddr {
    #[doc = " Creates an `IpAddr::V6` from an eight element 16-bit array."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " use std::net::{IpAddr, Ipv6Addr};"]
    #[doc = ""]
    #[doc = " let addr = IpAddr::from(["]
    #[doc = "     525u16, 524u16, 523u16, 522u16,"]
    #[doc = "     521u16, 520u16, 519u16, 518u16,"]
    #[doc = " ]);"]
    #[doc = " assert_eq!("]
    #[doc = "     IpAddr::V6(Ipv6Addr::new("]
    #[doc = "         0x20d, 0x20c,"]
    #[doc = "         0x20b, 0x20a,"]
    #[doc = "         0x209, 0x208,"]
    #[doc = "         0x207, 0x206"]
    #[doc = "     )),"]
    #[doc = "     addr"]
    #[doc = " );"]
    #[doc = " ```"]
    #[inline]
    fn from(segments: [u16; 8]) -> IpAddr {
        IpAddr::V6(Ipv6Addr::from(segments))
    }
}
