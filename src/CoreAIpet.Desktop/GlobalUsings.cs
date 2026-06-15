// 解决 WPF + WinForms 命名空间冲突
global using Application = System.Windows.Application;
global using MouseEventArgs = System.Windows.Input.MouseEventArgs;
global using KeyEventArgs = System.Windows.Input.KeyEventArgs;
global using Point = System.Windows.Point;
global using Size = System.Windows.Size;
global using Cursors = System.Windows.Input.Cursors;
global using ScaleTransform = System.Windows.Media.ScaleTransform;
global using FormsMouseEventArgs = System.Windows.Forms.MouseEventArgs;
global using FormsMouseButtons = System.Windows.Forms.MouseButtons;
