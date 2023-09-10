import random;


with open('Tests/test_data.txt', 'w') as f:
    for _ in range(1000000):\
        f.write(str(random.randint(-9223372036854775808, 9223372036854775807)) + '\n')
