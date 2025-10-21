# ==============================
# Self-Signed Code Signing Script for Ji Hoo Yoon
# Windows version (PowerShell)
# ==============================

# --- SETTINGS ---
$displayName = "Ji Hoo Yoon"
$exePath = "target\release\jisrot.exe"  # chỉnh nếu khác
$pfxPassword = Read-Host -AsSecureString "Enter password for .pfx"

Write-Host "=== Creating self-signed code signing certificate... ==="
$cert = New-SelfSignedCertificate `
    -Type CodeSigningCert `
    -Subject "CN=$displayName" `
    -CertStoreLocation "Cert:\CurrentUser\My" `
    -KeyUsage DigitalSignature `
    -KeyAlgorithm RSA `
    -KeyLength 4096 `
    -NotAfter (Get-Date).AddYears(5)

$thumb = $cert.Thumbprint
Write-Host "Certificate created with thumbprint: $thumb"

# --- Export files ---
$pfxPath = "$PWD\jihooyoon_signing.pfx"
$cerPath = "$PWD\jihooyoon_cert.cer"

Export-PfxCertificate -Cert "Cert:\CurrentUser\My\$thumb" -FilePath $pfxPath -Password $pfxPassword | Out-Null
Export-Certificate -Cert "Cert:\CurrentUser\My\$thumb" -FilePath $cerPath | Out-Null

Write-Host "Exported:"
Write-Host " - PFX: $pfxPath"
Write-Host " - CER: $cerPath"

# --- Sign executable ---
if (!(Test-Path $exePath)) {
    Write-Host "ERROR: File not found: $exePath" -ForegroundColor Red
    exit 1
}

Write-Host "=== Signing $exePath ==="
# Bạn có thể dùng /n "$displayName" nếu cert đã trong store
signtool sign /f $pfxPath /p (ConvertFrom-SecureString $pfxPassword -AsPlainText) /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 $exePath

# --- Verify ---
signtool verify /pa /v $exePath
Write-Host "=== Done! ==="

Write-Host "Next steps:"
Write-Host " 1. Import $cerPath into Trusted Root Certification Authorities on target machines."
Write-Host " 2. Windows will then trust your signed binaries."
