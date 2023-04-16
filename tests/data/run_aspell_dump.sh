#!/bin/bash
############################################################
# Help                                                     #
############################################################
Help()
{
    # Display Help
    echo "Generate a word list from aspell."
    echo
    echo "Syntax: run_aspell_dump.sh"
    echo
}

if [ $# != 0 ]; then
    Help
    exit 1
fi

aspell --ignore=2 -d en dump master | grep -vwE '\w{1,2}' | grep -v "\w*[-']" > aspell_dump.txt
