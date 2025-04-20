# Set environment variables
$env:RUSTFLAGS = "-C target-cpu=atmega328p"
$env:AVR_CPU_FREQUENCY_HZ = "16000000"

# Build with verbose output
Write-Host "Building with Cargo..."
cargo build -Z build-std=core --release --verbose 

# If build fails, try manual linking
if ($LASTEXITCODE -ne 0) {
    Write-Host "Cargo build failed, attempting manual linking..."
    
    # Path to the object file - find the most recent algae_medium_monitor*.o file
    $obj_file = (Get-ChildItem -Path "target\avr-atmega328p\release\deps" -Filter "algae_medium_monitor*.o" | Sort-Object LastWriteTime -Descending | Select-Object -First 1).FullName
    
    if ($obj_file) {
        Write-Host "Found object file: $obj_file"
        Write-Host "Last modified: $((Get-Item $obj_file).LastWriteTime)"
        
        # Manual link with avr-gcc - using simpler options
        Write-Host "Manually linking with avr-gcc..."
        avr-gcc -mmcu=atmega328p -o target\avr-atmega328p\release\algae-medium-monitor.elf $obj_file
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "Manual linking successful!"
            Write-Host "Output ELF file: target\avr-atmega328p\release\algae-medium-monitor.elf"
            
            # Generate hex file for flashing
            Write-Host "Generating HEX file for flashing..."
            avr-objcopy -O ihex -R .eeprom target\avr-atmega328p\release\algae-medium-monitor.elf target\avr-atmega328p\release\algae-medium-monitor.hex
            
            if ($LASTEXITCODE -eq 0) {
                Write-Host "HEX file generated successfully!"
                Write-Host "Output HEX file: target\avr-atmega328p\release\algae-medium-monitor.hex"
                
                # Display size information
                Write-Host "Size information:"
                avr-size target\avr-atmega328p\release\algae-medium-monitor.elf
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