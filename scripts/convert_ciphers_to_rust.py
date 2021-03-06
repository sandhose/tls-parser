#!/usr/bin/python

import sys

TABLE_NAME = "tls_ciphersuites"

def is_number(s):
    try:
        float(s)
        return True
    except ValueError:
        return False

def handle_line(line):
    #print line
    fields = line.split(':')
    # fixes
    rfc = fields[12]
    if not is_number(rfc):
        rfc = 0
    # enc
    if fields[5] == "3DES":
        enc = 'TripleDes'
    else:
        enc = fields[5].title()
    # au
    if "+" in fields[4]:
        au = fields[4].replace("+","_").title()
    else:
        au = fields[4].title()
    # mac
    mac = fields[8].title().replace("-","")
    print """        m.insert(0x%s,TlsCipherSuite{ name:"%s", id:0x%s, kx:TlsCipherKx::%s, au:TlsCipherAu::%s, enc:TlsCipherEnc::%s,  enc_mode:TlsCipherEncMode::%s, enc_size:%s, mac:TlsCipherMac::%s, mac_size:%s,});""" \
    % (fields[0],fields[1],fields[0],
            (fields[3] or "NULL").title(), # kx
            au, # au
            enc, # enc
            (fields[6] or "NULL").title(), # enc_mode
            fields[7], # enc_size
            mac, # mac
            fields[9], # mac_size
            )
#    print """
#  - cipher: %s
#    cs: 0x%s
#    name: %s
#    openssl-name: %s
#    kx: %s
#    au: %s
#    enc: %s
#    enc-mode: %s
#    enc-size: %s
#    mac: %s
#    mac-size: %s
#    prf: %s
#    prf-size: %s
#    rfc: %s
#    export: %s
#    minversion: 0x%s
#    maxversion: 0x%s""" % (fields[1],
#        fields[0], fields[1],
#        fields[2] or "NULL",
#        fields[3],
#        fields[4], fields[5],
#        fields[6] or "NULL",
#        fields[7],
#        fields[8], fields[9], fields[10], fields[11],
#        rfc, fields[13], fields[14], fields[15].rstrip(),
#        )

for line in sys.stdin:
    try:
        handle_line(line)
    except Exception,e:
        print e
        print line
        raise

print ""
