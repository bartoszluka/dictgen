# dictgen

## Usage

```console
./dict -f pl_full.txt -s pl.txt -o pl.dict --header "dictionary=lepszy:pl,locale=pl,description=ulepszony polski słownik od Bartka,date=$(date +%s),version=1"
```

where:

- `pl_full.txt` is a frequency file with the following format:

  ```
  nie 8583207  
  to 6394833  
  się 5144785  
  w 3988120  
  na 3386060  
  i 3249275  
  że 3140397  
  z 3104206  
  co 2834994  
  jest 2730378  
  ```

- `pl.txt` is a dictionary file used for spellchecking the `pl_full.txt` file, one word per line.

- `--header` value is a header for the `.dict` file.

  format:
  `dictionary=<name>:<lang>,locale=<lang>,description=<text>,date=<unix timestamp>,version=1`

This generates the file specified in `-o` parameter and can be directly loaded into the Android keyboard.
