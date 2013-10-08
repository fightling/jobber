Purpose
======

Command line tool for tracking work time.

Installation
============

You need ruby 1.8.7 to run this script.

Just copy the script into your bin and do a chmod to make it executable

    chmod u+x jobber.rb

Usage
=====

Start time tracking
-------------------

Start tracking now:

    jobber -s

or

    jobber -s now

Start tracking at specific time in past:
    
    jobber -s 12:00

Start tracking at absolute date and time:
    
    jobber -s 10/30/2013,12:00

Start tracking at relative time:
    
    jobber -s 5h-

End time tracking
-----------------

End tracking now:
    
    jobber -e

or
    
    jobber -e now

End tracking at specific time in past:
    
    jobber -e 14:00

End tracking at absolute date and time:
    
    jobber -e 10/30/2013,14:00

End tracking at relative time:
    
    jobber -s 1m+

List jobs
---------
List all known jobs:
    
    jobber -l

Monthly report:
    
    jobber -r

