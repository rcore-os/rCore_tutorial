import os
import sys
tests = {
    'labkernel': (False, 'test_test.rs'),
    'lab2': (False, 'pmm_test.rs'),
    'lab3': (False, 'vm_test.rs'),
    'labuser': (True, 'test_test.rs'),
    'lab5': (True, 'fork_test.rs'),
    'lab6': (True, 'stride_test.rs'),
    'lab7': (False, 'mutex_test.rs'),
    'lab8': (True, 'pipe_test.rs'),
}
if sys.argv[1] == 'clean':
    os.system('rm lab*')
    exit()
print('testing ' + sys.argv[1] + '...')
try:
    user_test, test_file = tests[sys.argv[1]]
    if user_test:
        # user_test
        # save process/mod.rs
        os.system('\\cp os/src/process/mod.rs os/src/process/mod_backup.rs')
        # replace with user test
        os.system('\\cp test/usr/' + test_file +
                  ' usr/rust/src/bin/' + test_file)
        s = open('os/src/process/mod.rs').read()
        s = s.replace('rust/user_shell', 'rust/' +
                      test_file[:test_file.find('.')])
        with open('os/src/process/mod.rs', 'w') as f:
            f.write(s)
        # try test
        c = os.system('make clean')
        c = os.system('make run > ' + sys.argv[1] + '.result')
        if c == 0:
            print('test successfully')
        else:
            print('test failed')
        print('see ' + sys.argv[1] + '.result')
        # remove user test
        os.system('rm usr/rust/src/bin/' + test_file)
        # backup process/mod.rs
        os.system('\\cp os/src/process/mod_backup.rs os/src/process/mod.rs')
        os.system('rm os/src/process/mod_backup.rs')
        # open result file
        if c == 0:
            os.system('cat ' + sys.argv[1] + '.result | less')
    else:
        # kernel test
        # save init.rs
        os.system('\\cp os/src/init.rs os/src/init_backup.rs')
        # replace with kernel test
        os.system('\\cp test/' + test_file + ' os/src/init.rs')
        # try test
        c = os.system('make run > ' + sys.argv[1] + '.result')
        if c == 0:
            print('test successfully')
        else:
            print('test failed')
        print('see ' + sys.argv[1] + '.result')
        # backup init.rs
        os.system('\\cp os/src/init_backup.rs os/src/init.rs')
        os.system('rm os/src/init_backup.rs')
        # open result file
        if c == 0:
            os.system('cat ' + sys.argv[1] + '.result | less')
except:
    print('Usage: python3 test.py labX/clean (X={2,3,5,6,7,8,kernel,user})')
