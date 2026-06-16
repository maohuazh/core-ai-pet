using System.IO;
using Microsoft.Extensions.Logging;
using CoreAIpet.Core.Interfaces;
using CoreAIpet.Desktop.Live2D.Rendering;
using CoreAIpet.Desktop.Services.Configuration;

namespace CoreAIpet.Desktop.Services.Live2D;

/// <summary>
/// Live2D 模型加载服务 — 模型加载、变体管理、路径验证、回退逻辑
/// </summary>
public class ModelLoaderService
{
    private readonly ConfigService _configService;
    private readonly Live2DRenderHost _renderHost;
    private readonly ILogger<ModelLoaderService> _logger;
    private bool _isLive2DActive;

    public bool IsLive2DActive => _isLive2DActive;
    public string? LastError { get; private set; }

    public ModelLoaderService(
        ConfigService configService,
        Live2DRenderHost renderHost,
        ILogger<ModelLoaderService> logger)
    {
        _configService = configService;
        _renderHost = renderHost;
        _logger = logger;
    }

    /// <summary>
    /// 加载配置的模型
    /// </summary>
    public bool LoadModel()
    {
        var modelPath = _configService.Live2D.ModelPath;
        var variant = _configService.Live2D.ModelVariant;
        return LoadVariant(modelPath, variant);
    }

    /// <summary>
    /// 加载指定变体
    /// </summary>
    public bool LoadVariant(string modelDirectory, string variantFilename)
    {
        LastError = null;

        // 验证目录存在
        if (!Directory.Exists(modelDirectory))
        {
            LastError = $"Model directory does not exist: {modelDirectory}";
            _logger.LogError(LastError);
            _isLive2DActive = false;
            return false;
        }

        // 构建完整路径
        var fullPath = Path.Combine(modelDirectory, variantFilename);

        // 验证文件存在
        if (!File.Exists(fullPath))
        {
            LastError = $"Model variant file does not exist: {fullPath}";
            _logger.LogError(LastError);
            _isLive2DActive = false;
            return false;
        }

        try
        {
            // 卸载当前模型
            _renderHost.UnloadModel();

            // 加载新模型
            var success = _renderHost.LoadModel(fullPath);
            if (success)
            {
                _isLive2DActive = true;
                _logger.LogInformation($"Successfully loaded Live2D model: {fullPath}");
            }
            else
            {
                LastError = "Bridge_LoadModel returned false";
                _logger.LogError(LastError);
                _isLive2DActive = false;
            }

            return success;
        }
        catch (Exception ex)
        {
            LastError = $"Exception loading model: {ex.Message}";
            _logger.LogError(ex, LastError);
            _isLive2DActive = false;
            return false;
        }
    }

    /// <summary>
    /// 获取可用的模型变体列表
    /// </summary>
    public List<ModelVariant> GetAvailableVariants()
    {
        var variants = new List<ModelVariant>();
        var modelPath = _configService.Live2D.ModelPath;

        if (!Directory.Exists(modelPath))
        {
            _logger.LogWarning($"Model directory does not exist: {modelPath}");
            return variants;
        }

        try
        {
            var files = Directory.GetFiles(modelPath, "*.json");
            foreach (var file in files)
            {
                var filename = Path.GetFileName(file);
                var displayName = ParseVariantDisplayName(filename);
                variants.Add(new ModelVariant
                {
                    Filename = filename,
                    DisplayName = displayName,
                    FullPath = file
                });
            }

            // 按显示名称排序
            variants.Sort((a, b) => string.Compare(a.DisplayName, b.DisplayName, StringComparison.OrdinalIgnoreCase));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error scanning model directory");
        }

        return variants;
    }

    /// <summary>
    /// 解析变体文件名为友好的显示名称
    /// </summary>
    private string ParseVariantDisplayName(string filename)
    {
        // 移除 .json 扩展名
        var name = Path.GetFileNameWithoutExtension(filename);

        // 特殊处理 default
        if (name == "model.default")
            return "Default";

        // 解析格式: model.YEAR.TYPE.VARIANT
        // 例如: model.2016.xmas.1 -> Christmas 2016 Variant 1
        var parts = name.Split('.');
        if (parts.Length >= 2 && parts[0] == "model")
        {
            var year = parts[1];
            var type = parts.Length > 2 ? parts[2] : "";
            var variant = parts.Length > 3 ? parts[3] : "";

            // 类型映射
            var typeDisplay = type.ToLowerInvariant() switch
            {
                "xmas" => "Christmas",
                "newyear" => "New Year",
                "summer" => "Summer",
                "school" => "School",
                "cba-normal" => "CBA Normal",
                "cba-super" => "CBA Super",
                "tomo-bukatsu" => "Tomobukatsu",
                "valley" => "Valley",
                "vdays" => "Valentine's Day",
                "lover" => "Lover",
                "spring" => "Spring",
                "bls-summer" => "BLS Summer",
                "bls-winter" => "BLS Winter",
                _ => type
            };

            // 构建显示名称
            var displayName = typeDisplay;
            if (!string.IsNullOrEmpty(year) && year != "default")
                displayName += $" {year}";
            if (!string.IsNullOrEmpty(variant))
                displayName += $" Variant {variant}";

            return displayName;
        }

        // 默认返回原始名称
        return name;
    }
}

/// <summary>
/// 模型变体信息
/// </summary>
public class ModelVariant
{
    public string Filename { get; set; } = "";
    public string DisplayName { get; set; } = "";
    public string FullPath { get; set; } = "";
}
