#!/bin/bash

if [ ! -d ./dumps ] ; then
    mkdir dumps
fi

hexdump -C Zork1.z3 > dumps/Zork1.hexdump
txd -d -n Zork1.z3 > dumps/Zork1.txd
infodump -f Zork1.z3 > dumps/Zork1.infodump

