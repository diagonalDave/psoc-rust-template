echo off
echo Sign CM0p elf file.
"C:\Program Files (x86)\Cypress\PDL\3.1.3\tools\win\elf\cymcuelftool.exe" --sign "C:\Users\Dave\Documents\fossils\rust\projects\psoc6_poc\cm0p\target\thumbv6m-none-eabi\debug\cm0p" --output "C:\Users\Dave\Documents\fossils\rust\projects\psoc6_poc\cm0p\target\thumbv6m-none-eabi\debug\cm0p.elf" --hex "C:\Users\Dave\Documents\fossils\rust\projects\psoc6_poc\cm0p\target\thumbv6m-none-eabi\debug\cm0p.hex"

echo Sign CM4f elf file
"C:\Program Files (x86)\Cypress\PDL\3.1.3\tools\win\elf\cymcuelftool.exe" --sign "C:\Users\Dave\Documents\fossils\rust\projects\psoc6_poc\cm4f\target\thumbv7em-none-eabihf\debug\cm4f" --output "C:\Users\Dave\Documents\fossils\rust\projects\psoc6_poc\cm4f\target\thumbv7em-none-eabihf\debug\cm4f_signed.elf" --hex "C:\Users\Dave\Documents\fossils\rust\projects\psoc6_poc\cm4f\target\thumbv7em-none-eabihf\debug\cm4f_signed.hex"

echo Merge elf files.
"C:\Program Files (x86)\Cypress\PDL\3.1.3\tools\win\elf\cymcuelftool.exe" --merge  "C:\Users\Dave\Documents\fossils\rust\projects\psoc6_poc\cm4f\target\thumbv7em-none-eabihf\debug\cm4f_signed.elf" "C:\Users\Dave\Documents\fossils\rust\projects\psoc6_poc\cm0p\target\thumbv6m-none-eabi\debug\cm0p.elf" --output "C:\Users\Dave\Documents\fossils\rust\projects\psoc6_poc\bin\dual_minimum.elf" --hex "C:\Users\Dave\Documents\fossils\rust\projects\psoc6_poc\bin\dual_minimum.hex"

echo Create objdump files
arm-none-eabi-objdump -D -C bin\dual_minimum.elf > bin\dual_minimum_objdump.txt

echo Create readelf file
readelf -a bin\dual_minimum.elf > bin\dual_minimum_readelf.txt
echo All done.

