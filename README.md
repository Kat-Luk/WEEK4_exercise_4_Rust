Task 4

Let's continue with threads. The user has either $100,000 or $1,000,000 worth of money and must stop two thieves from stealing it. The program emphasizes data transfer between threads. It has three threads, two of which represent thieves. The last one is ready to receive input from the user, allowing him to stop criminals before more money is lost. The end result is that the thieves got away with the money they took before you typed "catch" or you lost all your money before you could type "catch".

Make a function called create_threads() that creates a thread per thief, and one thread where the user catches the thieves and loops until the user says "catch" or runs out of money. The function takes a number as a parameter, not a reference.

Thief threads have a sleep time of 5 seconds for a thief who takes $10,000 and 3 seconds for another thief who takes $35,000 from you. If the remaining amount of money is $600,000 or less and the starting amount was over it originally, the thief who takes $35,000 does no longer steal.


Example run, $1 million:

Do you have a million dollars? | y = yes, n = no

 

y

 

All right then, millionaire.

 

ALERT!!! Someone stole $35,000 from you!

 

Funds left: 965000

 

ALERT!!! Someone stole $10,000 from you!

 

Funds left: 955000

 

catch

 

The thieves have left.

 

Example run, no million dollars:

Do you have a million dollars? | y = yes, n = no

 

n

 

Let's just assume you have $100,000 then.

 

ALERT!!! Someone stole $35,000 from you!

 

Funds left: 65000

 

ALERT!!! Someone stole $10,000 from you!

 

Funds left: 55000

 

ALERT!!! Someone stole $35,000 from you!

 

Funds left: 20000

 

catch

 

The thieves have left.
