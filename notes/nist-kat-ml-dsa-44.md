
## Automated Cryptographic Validation Test System - Gen/Vals
https://github.com/usnistgov/ACVP-Server

The list of all KATs in JSON:
https://github.com/usnistgov/ACVP-Server/tree/master/gen-val/json-files

We import the vectors in this file to test our implementation:
https://github.com/usnistgov/ACVP-Server/blob/master/gen-val/json-files/ML-DSA-keyGen-FIPS204/internalProjection.json

For more information, see: https://pages.nist.gov/ACVP/draft-celi-acvp-ml-dsa.html

## Steps to Prepare KAT File
```bash
$ pwd
/Users/dprasad/projects/dicp-local/ml-dsa-44/kats

$ python3 nist-fips204-kats.py 
Written output to nist-acvp-keygen-kats.txt

$ 

```