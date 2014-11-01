#!/usr/bin/env python
# -*- coding: utf-8 -*-

from setuptools import setup

setup(
    name="AGFW",
    version="0.1",
    description="Across the Great Fire Wall.",
    author="Liam",
    author_email="liamchzh@gmail.com",
    url="https://github.com/liamchzh/AGFW",
    packages=['AGFW'],
    install_requires=[],
    entry_points={
        'console_scripts': [
            'AGFW = AGFW.server:main',
            'FGFW = AGFW.local:main',
        ]
    },
)
