# ADIF to REG1TEST converter
ADIF file format (.adi) to REG1TEST file format (.edi) command line (CLI) converter.
Usage is extremely simple - just provide ADIF file as argument to the **`adi2edi`** 
executable and hit Enter. The results in REG1TEST format are output to terminal. 
Need a file, just add **`-f`** and results will be saved to a file with the same 
file name, but .edi extension in place of .adi. Too verbose? No problem, add 
**`-s`** to skip remarks. Need more, type **`-h`** for help.
## Release 0.4.0
Added functionality to split results by band. If ADIF file records contain 
different bands, the REG1TEST output is grouped by each band. When such output 
is redirected to file, each band QSOs are stored to individual file 
appended by band suffix.
