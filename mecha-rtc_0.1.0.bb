# Auto-Generated by cargo-bitbake 0.3.16
#
inherit cargo

# If this is git based prefer versioned ones if they exist
# DEFAULT_PREFERENCE = "-1"

# how to get mecha-rtc could be as easy as but default to a git checkout:
# SRC_URI += "crate://crates.io/mecha-rtc/0.1.0"
SRC_URI += "git://github.com/Dhruvesh08/mecha-rtc;protocol=https;nobranch=1;branch=main"
SRCREV = "eeafecf1ea860090af6a56b8406e9f76ae3f96e1"
S = "${WORKDIR}/git"
CARGO_SRC_DIR = ""
PV:append = ".AUTOINC+eeafecf1ea"

# please note if you have entries that do not begin with crate://
# you must change them to how that package can be fetched
SRC_URI += " \
"



# FIXME: update generateme with the real MD5 of the license file
LIC_FILES_CHKSUM = " \
    "

SUMMARY = "mecha-rtc"
HOMEPAGE = "https://github.com/Dhruvesh08/mecha-rtc"
LICENSE = "CLOSED"

# includes this file if it exists but does not fail
# this is useful for anything you may want to override from
# what cargo-bitbake generates.
include mecha-rtc-${PV}.inc
include mecha-rtc.inc
