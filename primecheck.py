#664579
import sys
import math
import timeit
def primenumber(topt):
  try:
    top = int(topt[1])
  except:
    top = int(input("enter max num: "))
  liss = []
  if top <3:
    print("0 is not prime, 1 is not prime")
    exit()
  liss.append(2)
  start = timeit.default_timer()
  for i in range(3,top,2):
    prime = True
    #q = 0
    sq = math.sqrt(i)
    #if sq.is_integer() == True:
      #prime = False
      #q = 1
    
    #print(i)
    #for e in range(2,i):
    #if q == 0:
    for x in liss:
      if x > sq:
        break
      if i %x ==0:
      #if i %x ==0 and q == 1:
        prime = False
        break
    if prime == True:
      #print(prime, i)
      liss.append(i)
      #print(i)
  stop = timeit.default_timer()
  #you can add up prime numbers and get one more than a prime number?

  
  #if liss == check:
  #  print("valid")
  print("total" + str(len(liss)))
  
  #import timeit
  
  #start = timeit.default_timer()
  
  #Your statements here
  
  #stop = timeit.default_timer()
  
  print('Time: ', stop - start)

try:
    reps = int(sys.argv[2])
except:
    reps = int(input("Reps: "))
for i in range(reps):
    primenumber(sys.argv)


#1 mill
#with is int
  #15.481959559999723
  #11.959761623999839
  #11.588943898000252
  #13.170005102999767
  #10.630825021999954
#without
  #11.911473774000115
  #9.360055898000155
  #11.457596152000406
  #10.007674079000026
  #9.688849957999992

#10 Mill


#without
  #257.70610118199875
  #255.7370110169977
  #255.57673045299816
  #253.80866700800107
  #251.4775341539971

  #201.2106872880031
  #206.95720532799896
#with
  #210.96130497700506
  #211.1541725030038
  #213.60866672499833
  #228.48060699299822
  #202.73826782399556