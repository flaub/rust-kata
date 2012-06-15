#!/usr/bin/env python

import os
from fabricate import *

def init():
	run('mkdir', 'bin')

def build():
	init()
	run('rustc', 'chop.rs', '-o', 'bin/chop')

def test():
	init()
	run('rustc', 'chop.rs', '-o', 'bin/test', '--test')
	
def clean():
	autoclean()
	run('rm', '-rf', 'bin')

main()
