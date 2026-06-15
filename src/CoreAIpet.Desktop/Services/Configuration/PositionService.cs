using CoreAIpet.Core.Interfaces;
using CoreAIpet.Core.Models.Settings;

namespace CoreAIpet.Desktop.Services.Configuration;

/// <summary>
/// 窗口位置持久化 — 通过 ConfigService 读写位置配置
/// </summary>
public class PositionService : IPositionService
{
    private readonly ConfigService _configService;

    public PositionService(ConfigService configService)
    {
        _configService = configService;
    }

    public Task<WindowPosition?> LoadAsync()
    {
        var pos = _configService.AppSettings.Position;
        if (pos.X == 0 && pos.Y == 0)
            return Task.FromResult<WindowPosition?>(null);

        return Task.FromResult<WindowPosition?>(new WindowPosition(pos.X, pos.Y));
    }

    public async Task SaveAsync(WindowPosition position)
    {
        _configService.AppSettings.Position = new PositionConfig
        {
            X = position.X,
            Y = position.Y
        };
        await _configService.SaveAsync();
    }
}
