#!/usr/bin/env python

import os
from fabricate import *

def init():
	run('mkdir', 'bin')

def build():
	init()
	run('rustc', 'chop1.rs', '-o', 'bin/chop1')

def test():
	init()
	run('rustc', 'chop1.rs', '--test', '-o', 'bin/test_chop1')
	
def clean():
	autoclean()
	run('rm', '-rf', 'bin')

main()
