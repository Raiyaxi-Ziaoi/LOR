
all:
	rustc lnrc.rs
	- rm -f lnrc.pdb
	- rm -f .gitattributes