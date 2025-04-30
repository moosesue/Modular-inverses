def standard_gcd(a: int, b: int)->int:
    return standard_gcd(b%a,a) if a else b

def extended_gcd(a: int, b: int, positive_x: bool)->tuple[int,int,int]:

    original_a = a
    original_b = b
    
    #we want to find x and y such that ax + by = gcd(a,b)
    #If b > a, the output results will naturally swap and the signs may also change.
   
    #previous x and y
    x0, y0 = 1,0 #represents a when x = 1 and y = 0
    #current x and y
    x1, y1 = 0,1 #represents b when x = 0 and y = 1

    while b !=0: #while the remainder isn't 0......
        #This part is same as ordinary GCD algorithm. 
        #quotient = whole number floor result of a divided by b
        quotient = a//b
        #set a to b and at same time set remainder b = a mod b
        a, b = b, a % b
        
        #This part for finding the coefficients by setting current values based on previous values.
        #we are keeping track of expressing each remainder in terms of current a and b.
        #This is why we are referring back to the previous values.
        #current x0 becomes last x1 and current x1 becomes last x0 - current quotient x last x1
        #This is because the new remainder is previous a - quotient x previous b
        x0, x1 = x1, x0 - quotient * x1
        #current y0 becomes last y1 and current y1 becomes last y0 - current quotient x last y1
        y0, y1 = y1, y0 - quotient * y1

    #if we want positive x but we have negative x as a result, we can use:
    #x = x0 + k*(b/gcd)
    #y = y0 + k*(a/gcd)
    #k will represent a shift of multiples of b/gcd (or would be a/gcd if we wanted positive y)
    #needed if we are wanting ax = 1 (mod m) where m = original_b
    if (positive_x and x0 < 0):
       #shift of k calculates how much needs to be added in one go where original_b//a is size of step needed
       #use (original_b//a) - 1 to make sure we get shift rounded up so x0 becomes positive.
       shift_k = (abs(x0) + (original_b // a) - 1) // (original_b // a)  # smallest integer needed to make x positive
       x0 += shift_k*(original_b//a) #a = gcd
       y0 -= shift_k*(original_a//a)
            
            
    return a,x0,y0
