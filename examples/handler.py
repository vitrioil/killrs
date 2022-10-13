# To simulate handling of SIGTERM
import os
import signal
import time

class Handler:
    def __init__(self):
        signal.signal(signal.SIGINT, self.handle)
        signal.signal(signal.SIGTERM, self.handle)

    def handle(self, *args):
        # do nothing
        pass

if __name__ == '__main__':
    print(os.getpid())
    Handler()
    while True:
        time.sleep(1)
