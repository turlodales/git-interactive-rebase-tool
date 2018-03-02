#!/usr/bin/env bash

cd "c:\"

# download and build pdcurses with patches
if [[ ! -d "c:\PDCurses-3.4" ]]; then
	wget 'https://downloads.sourceforge.net/project/pdcurses/pdcurses/3.4/PDCurses-3.4.tar.gz'
	tar vxf PDCurses-3.4.tar.gz
	make -C PDCurses-3.4/win32/ -f gccwin32.mak WIDE=Y
	mv PDCurses-3.4/win32/pdcurses.a PDCurses-3.4/win32/libpdcurses.a
fi
