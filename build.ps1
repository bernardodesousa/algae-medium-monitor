# Set environment variables
$env:RUSTFLAGS = "-C target-cpu=atmega328p"
$env:AVR_CPU_FREQUENCY_HZ = "16000000"

# Build with verbose output
Write-Host "Building with Cargo..."
cargo build -Z build-std=core --release --verbose 

# If build fails, try manual linking
if ($LASTEXITCODE -ne 0) {
    Write-Host "Cargo build failed, attempting manual linking..."
    
    # Path to the object file - find the most recent blink*.o file
    $obj_file = (Get-ChildItem -Path "target\avr-atmega328p\release\deps" -Filter "blink*.o" | Sort-Object LastWriteTime -Descending | Select-Object -First 1).FullName
    
    if ($obj_file) {
        Write-Host "Found object file: $obj_file"
        Write-Host "Last modified: $((Get-Item $obj_file).LastWriteTime)"
        
        # Manual link with avr-gcc - going back to the working version
        Write-Host "Manually linking with avr-gcc..."
        avr-gcc -mmcu=atmega328p -o target\avr-atmega328p\release\blink.elf $obj_file -lgcc -lc
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "Manual linking successful!"
            Write-Host "Output ELF file: target\avr-atmega328p\release\blink.elf"
            
            # Examine the symbols in the ELF file
            Write-Host "Checking ELF file symbols:"
            avr-nm target\avr-atmega328p\release\blink.elf | Select-String "main"
            
            # Generate hex file for flashing
            Write-Host "Generating HEX file for flashing..."
            avr-objcopy -O ihex -R .eeprom target\avr-atmega328p\release\blink.elf target\avr-atmega328p\release\blink.hex
            
            if ($LASTEXITCODE -eq 0) {
                Write-Host "HEX file generated successfully!"
                Write-Host "Output HEX file: target\avr-atmega328p\release\blink.hex"
                
                # Display size information
                Write-Host "Size information:"
                avr-size target\avr-atmega328p\release\blink.elf
                
                # Verify hex file starts with proper boot vector
                Write-Host "First 10 lines of HEX file:"
                Get-Content target\avr-atmega328p\release\blink.hex | Select-Object -First 10
            } else {
                Write-Host "HEX file generation failed."
            }
        } else {
            Write-Host "Manual linking failed."
        }
    } else {
        Write-Host "Could not find object file for manual linking."
    }
} 