a = 0
b = 1

for i = 1, 101 do
    c = a + b
    print("Number: ", i, " of the fibonacci sequence is: ", c)
    a = b
    b = c
end