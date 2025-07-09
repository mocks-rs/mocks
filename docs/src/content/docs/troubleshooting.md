---
title: Troubleshooting
description: Common issues and solutions when using mocks
---

## Common Issues and Solutions

### 1. Server Won't Start

#### Issue: "address already in use" Error

```
Error: Failed to bind to address: Address already in use (os error 98)
```

**Cause**: The specified port (default 3000) is already in use

**Solutions**:
- Use a different port
```bash
mocks -p 8080 storage.json
```

- Kill the existing process
```bash
# Find process using port 3000
lsof -i :3000
# Kill the process
kill -9 <PID>
```

#### Issue: "No such file or directory" Error

```
Error: No such file or directory (os error 2)
```

**Cause**: The specified JSON file doesn't exist

**Solutions**:
- Verify the file path
- Check if the file exists
```bash
ls -la storage.json
```

### 2. JSON File Issues

#### Issue: "Invalid JSON" Error

```
Error: Invalid JSON in storage file
```

**Cause**: JSON file has syntax errors

**Solutions**:
- Validate JSON syntax
- Use an online JSON validator
- Check for common errors:
  - Missing commas
  - Unmatched brackets
  - Incorrect quotation marks

#### Issue: "Duplicate resource names" Error

```
Error: Duplicate resource names found
```

**Cause**: JSON file contains duplicate resource names

**Solutions**:
- Make resource names unique
- Example: Cannot have both `api/v1/users` and `api/v2/users`

### 3. API Request Issues

#### Issue: 404 Not Found Error

**Causes**: 
- Accessing non-existent endpoints
- Using non-existent resource IDs

**Solutions**:
- Verify endpoint paths
- Check if resource IDs are correct
- Review available endpoints

#### Issue: 400 Bad Request Error

**Causes**: 
- Sending invalid JSON data
- Using incorrect search query format

**Solutions**:
- Validate request body JSON
- Check search query format (must be `field.matchtype=value`)

### 4. Search Functionality Issues

#### Issue: Search Not Working

**Causes**: 
- Incorrect search query format
- Trying to search on single object resources

**Solutions**:
- Use correct search format: `field.matchtype=value`
- Only use search on array-type resources
- Use supported match types (`exact`, `contains`, `startswith`, `endswith`)

**Correct example**:
```bash
curl "http://localhost:3000/posts?title.contains=post"
```

**Incorrect example**:
```bash
curl "http://localhost:3000/posts?title=post"  # Missing match type
```

### 5. Data Persistence Issues

#### Issue: Changes Not Saved

**Causes**: 
- `--no-overwrite` option is specified
- No file write permissions

**Solutions**:
- Remove `--no-overwrite` option
- Check file write permissions
```bash
ls -la storage.json
chmod 644 storage.json
```

#### Issue: Original JSON File Corrupted

**Causes**: 
- Unexpected program termination
- Invalid data writes

**Solutions**:
- Use debug mode to create backups
```bash
MOCKS_DEBUG_OVERWRITTEN_FILE=storage.debug.json mocks storage.json
```

- Regularly backup JSON files
```bash
cp storage.json storage.backup.json
```

### 6. Performance Issues

#### Issue: Slow Response Times

**Causes**: 
- Large JSON files
- Complex search queries

**Solutions**:
- Reduce JSON file size
- Remove unnecessary data
- Use more specific search conditions

### 7. CORS Issues

#### Issue: Browser Requests Failing

**Cause**: Cross-Origin Resource Sharing (CORS) restrictions

**Solutions**:
- For development, disable browser CORS restrictions
- Use a proxy server
- Configure reverse proxy with nginx or Apache

### 8. Debug Tips

#### Check Logs

mocks outputs logs to stdout:

```bash
mocks storage.json
```

#### Use Debug Mode

```bash
MOCKS_DEBUG_OVERWRITTEN_FILE=storage.debug.json mocks storage.json
```

#### Enable Verbose Logging

```bash
RUST_LOG=debug mocks storage.json
```

#### Health Check

Verify server is running:

```bash
curl -i http://localhost:3000/_hc
```

### 9. Best Practices

1. **Backup JSON Files**
   - Regularly backup important data

2. **Version Control**
   - Keep JSON files in Git
   - Add `*.debug.json` to `.gitignore`

3. **Separate Test Data**
   - Keep production and test data separate
   - Use appropriate file naming (e.g., `storage.test.json`)

4. **Resource Design**
   - Use unique resource names
   - Design proper ID structures

### 10. Getting Help

If issues persist, you can seek help through:

- **GitHub Issues**: [mocks-rs/mocks](https://github.com/mocks-rs/mocks/issues)
- **Version Check**: `mocks --version`
- **System Info**: OS, Rust version (if applicable)

When reporting issues, please include:
- mocks version
- Operating system
- Error messages
- JSON file (if possible)
- Commands executed

### 11. Environment Variables

#### MOCKS_DEBUG_OVERWRITTEN_FILE

Save modified data to a separate file:

```bash
MOCKS_DEBUG_OVERWRITTEN_FILE=storage.debug.json mocks storage.json
```

#### RUST_LOG

Enable debug logging:

```bash
RUST_LOG=debug mocks storage.json
```

### 12. Common CLI Mistakes

#### Wrong Host Binding

```bash
# Wrong: Will only bind to localhost
mocks storage.json

# Right: For Docker or external access
mocks -H 0.0.0.0 storage.json
```

#### File Path Issues

```bash
# Wrong: Relative path might not work
mocks ../data/storage.json

# Right: Use absolute path or correct relative path
mocks /full/path/to/storage.json
```

### 13. JSON Structure Guidelines

#### Valid Structure

```json
{
  "users": [
    { "id": "1", "name": "John" }
  ],
  "profile": { "id": "1", "name": "App" }
}
```

#### Invalid Structure

```json
{
  "users": [
    { "name": "John" }  // Missing required ID
  ],
  "api/v1/users": [     // Duplicate resource name
    { "id": "1", "name": "Jane" }
  ]
}
```

Remember that proper JSON structure and unique resource names are essential for mocks to work correctly.