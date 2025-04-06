# Flash the compiled Rust program to Arduino Pro Mini
# Check if COM port is set in environment variable
if (-not $env:AVR_COM_PORT) {
    # Prompt for COM port if not set
    $env:AVR_COM_PORT = Read-Host -Prompt "Enter COM port for Arduino (e.g. COM7)"
}

Write-Host "Flashing blink.hex to Arduino Pro Mini on $env:AVR_COM_PORT..."

# First rebuild to ensure we have the latest version
Write-Host "Rebuilding with updated code..."
./build.ps1

# Check if build was successful
if ($LASTEXITCODE -eq 0) {
    # Flash the hex file using avrdude
    Write-Host "`nPreparing to upload to Arduino Pro Mini..."
    Write-Host "IMPORTANT: Press the RESET button on Arduino about half a second after you see the avrdude command start" -ForegroundColor Yellow
    Write-Host "Press Enter when ready to begin upload process..." -ForegroundColor Green
    Read-Host
    
    Write-Host "Running avrdude now - be ready to press RESET in half a second!" -ForegroundColor Red
    avrdude -p atmega328p -c arduino -P $env:AVR_COM_PORT -b 57600 -D -U flash:w:target\avr-atmega328p\release\blink.hex:i
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "`nSuccess! Your Arduino should now be running the Rust program." -ForegroundColor Green
        Write-Host "You should see the Morse code for 'RUST' blinking on the LED." -ForegroundColor Green
        Write-Host "RUST in Morse: .-. ..- ... -"
    } else {
        Write-Host "`nUpload failed. Let's try once more with your reset timing approach."
        Write-Host "Press Enter when ready to try again..." -ForegroundColor Green
        Read-Host
        
        Write-Host "Running avrdude now - be ready to press RESET in half a second!" -ForegroundColor Red
        avrdude -p atmega328p -c arduino -P $env:AVR_COM_PORT -b 57600 -D -U flash:w:target\avr-atmega328p\release\blink.hex:i
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "`nSuccess! Your Arduino should now be running the Rust program." -ForegroundColor Green
            Write-Host "You should see the Morse code for 'RUST' blinking on the LED." -ForegroundColor Green
            Write-Host "RUST in Morse: .-. ..- ... -"
        } else {
            Write-Host "`nUpload failed again. Please check connections and try different timing with the reset button."
        }
    }
} else {
    Write-Host "Error: Build failed. Cannot proceed with flashing."
}
