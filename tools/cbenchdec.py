import sys
import re
from collections import OrderedDict


render_vers = re.compile(r'=== ./rust-raytracer-(\S+) (\S+) ===')
render_time = re.compile(r'Render done at \d+ \((\d+)s\)\.\.\.')


def process(stream_in):
    mode = 'lfvers'
    (version, jobname) = (None, None)
    for line in stream_in:
        line = line.strip()
        if mode == 'lfvers':
            vers_match = render_vers.match(line)
            if vers_match:
                (version, jobname) = vers_match.groups()
                mode = 'lftime'
        elif mode == 'lftime':
            time_match = render_time.match(line)
            if time_match:
                (time, ) = time_match.groups()
                yield (version, jobname, int(time))
                mode = 'lfvers'
        else:
            raise Exception()


if __name__ == '__main__':
    output = OrderedDict()
    for (version, jobname, time) in process(sys.stdin):
        # print("{}\t{}\t{}".format(version, jobname, time))
        if (version, jobname) not in output:
            add_to_list = list()
            output[(version, jobname)] = add_to_list
        else:
            add_to_list = output[(version, jobname)]
        add_to_list.append(time)

    for ((version, jobname), value) in output.items():
        print("{version} {jobname}\t{rest}".format(
            version=version,
            jobname=jobname,
            rest=' '.join(map(str, value))))
