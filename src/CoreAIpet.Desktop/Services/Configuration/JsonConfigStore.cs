using System.IO;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace CoreAIpet.Desktop.Services.Configuration;

/// <summary>
/// JSON 配置文件原子读写 — 带文件锁
/// </summary>
public class JsonConfigStore
{
    private readonly string _filePath;
    private readonly SemaphoreSlim _lock = new(1, 1);
    private static readonly JsonSerializerOptions JsonOptions = new()
    {
        WriteIndented = true,
        DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase
    };

    public JsonConfigStore(string filePath)
    {
        _filePath = filePath;
    }

    public async Task<T> LoadAsync<T>() where T : class, new()
    {
        await _lock.WaitAsync();
        try
        {
            if (!File.Exists(_filePath))
                return new T();

            var json = await File.ReadAllTextAsync(_filePath);
            return JsonSerializer.Deserialize<T>(json, JsonOptions) ?? new T();
        }
        finally
        {
            _lock.Release();
        }
    }

    public async Task SaveAsync<T>(T config) where T : class
    {
        await _lock.WaitAsync();
        try
        {
            var dir = Path.GetDirectoryName(_filePath);
            if (!string.IsNullOrEmpty(dir) && !Directory.Exists(dir))
                Directory.CreateDirectory(dir);

            var json = JsonSerializer.Serialize(config, JsonOptions);
            var tempPath = _filePath + ".tmp";
            await File.WriteAllTextAsync(tempPath, json);

            // 原子替换
            if (File.Exists(_filePath))
            {
                File.Replace(tempPath, _filePath, null);
            }
            else
            {
                File.Move(tempPath, _filePath);
            }
        }
        finally
        {
            _lock.Release();
        }
    }
}
