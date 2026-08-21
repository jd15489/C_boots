import re
import pandas as pd

from pathlib import Path

def read_log(file: Path) -> pd.DataFrame:
    data = []
    names = []
    results = False
    with file.open('r') as log_file:
        for line in log_file:
            if 'RESULTS' in line:
                results = True
                continue
            elif 'A V E R A G E S' in line or 'TIMINGS' in line:
                results = False
                break

            if results:
                line = re.split(r"\[.*?\]|\(.*?\)|[\s=]+", line)
                while '' in line:
                    line.remove('')
                if 'NSTEP' in line:
                    data.append([])
                    names.append([])
                    data[-1].extend(line[1::2])
                    names[-1].extend(line[::2])
                    continue
                elif 'Etot' in line:
                    data[-1].extend(line[1::2])
                    names[-1].extend(line[::2])
                    continue
                elif 'EKCMT' in line:
                    data[-1].append(line[5])
                    names[-1].append(line[4])
                
    data = pd.DataFrame(data, columns=names[0], dtype=float)
    return data

def save_internal_energy_to_file(output_csv: Path, amber_output: Path):
    data = read_log(amber_output)['Etot']
    data.to_csv(output_csv)
