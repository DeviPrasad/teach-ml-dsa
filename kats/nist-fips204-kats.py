import json

def write_mldsa_kat_text(json_file, output_file):
    with open(json_file, "r") as f:
        data = json.load(f)

    with open(output_file, "w") as out:
        for group in data.get("testGroups", []):
            if group.get("tgId") == 1:
                for test in group.get("tests", []):
                    out.write(f"tid = {test['tcId']:02}\n")
                    out.write(f"xi = {test['seed']}\n")
                    out.write(f"pk = {test['pk']}\n")
                    out.write(f"sk = {test['sk']}\n\n")

if __name__ == "__main__":
    json_file = "acvp-ml-dsa-keygen-fips204.json"
    output_file = "nist-acvp-keygen-kats.txt"
    write_mldsa_kat_text(json_file, output_file)
    print(f"Written output to {output_file}")
