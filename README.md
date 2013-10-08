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

    $ jobber -s
    jobber - job time tracker
    Starting new job:
        Pos: 1
      Start: Tue Oct 08 2013, 07:51

Start tracking at specific time in past:
    
    $ jobber -s 12:00

Start tracking at absolute date and time:
    
    $ jobber -s 10/30/2013,12:00

Start tracking at relative time:
    
    $ jobber -s 5h-

End time tracking
-----------------

End tracking now:
    
    $ jobber -e

End tracking at specific time in past:
    
    $ jobber -e 14:00

End tracking at absolute date and time:
    
    $ jobber -e 10/30/2013,14:00

End tracking at relative time:
    
    $ jobber -e 1m+

List jobs
---------
List all known jobs:
    
    $ jobber -l

Monthly report:
    
    $ jobber -r
    jobber - job time tracker
                             4/2013                         
         sun     mon     tue     wed     thu     fri     sat
               11.75       -   12.75       -       -       -
           -       -     4.5     6.5       -     7.5       -
           -       -       -       -       -       -       -
           -       -       -       -       -       -       -
           -       -       -
                   Monthly total: 43.0 hours                

    Total over all: 43.0 hours

