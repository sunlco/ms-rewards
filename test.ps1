# Function to generate a random number between 2 and 3
function Get-RandomSleep {
    return Get-Random -Minimum 2 -Maximum 4
}

# Check if ADB is available
try {
    adb version | Out-Null
} catch {
    Write-Host "ADB is not installed or not in PATH."
    exit
}

# Check if device is connected
$devices = adb devices | Select-String "device$"
if ($devices.Count -eq 0) {
    Write-Host "No device connected."
    exit
}

# Loop 1000 times
for ($i = 1; $i -le 1000; $i++) {
    Write-Host "Performing swipe $i"
    # Perform a swipe gesture from (x1, y1) to (x2, y2)
    adb shell input swipe 500 1500 500 500

    # Get a random sleep duration
    $sleepDuration = Get-RandomSleep

    Write-Host "Sleeping for $sleepDuration seconds"
    # Sleep for the random duration
    Start-Sleep -Seconds $sleepDuration
}