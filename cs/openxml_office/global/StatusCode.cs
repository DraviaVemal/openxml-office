// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.

using System;
using System.Data;
using System.IO;
using System.Runtime.InteropServices;

namespace draviavemal.openxml_office.global_2007
{
    public enum StatusCodeValues : sbyte
    {
        UnknownError = -1,
        Success = 0,
        InvalidArgument = 1,
        FlatBufferError = 2,
        FileNotFound = 3,
        IoError = 4,
    }
    public static class StatusCode
    {
        public static void ProcessStatusCode(sbyte statusCode, IntPtr errorMsg)
        {
            StatusCodeValues status = (StatusCodeValues)statusCode;
            string errorMessage = "Unknown Error";
            if (statusCode != 0)
            {
                errorMessage = Marshal.PtrToStringAnsi(errorMsg) ?? errorMessage;
            }
            switch (status)
            {
                case StatusCodeValues.Success:
                    break;
                case StatusCodeValues.InvalidArgument:
                    throw new ArgumentException(errorMessage);
                case StatusCodeValues.FlatBufferError:
                    throw new DataException(errorMessage);
                case StatusCodeValues.FileNotFound:
                    throw new FileNotFoundException(errorMessage);
                case StatusCodeValues.IoError:
                    throw new FileLoadException(errorMessage);
                default:
                    throw new Exception(errorMessage);
            }
        }
    }
}
