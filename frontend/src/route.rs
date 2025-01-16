use super::*;

#[ derive (Clone, Debug, Eq, PartialEq, Routable) ]
pub enum Route {
	#[ at ("/") ]
	Home,
	#[ at ("/dhcp-leases") ]
	DhcpLeases,
	#[ at ("/interfaces") ]
	Interfaces,
	#[ not_found ]
	#[ at ("/404") ]
	NotFound,
}
