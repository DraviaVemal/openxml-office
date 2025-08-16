using System;
using System.Runtime.InteropServices;
using Google.FlatBuffers;
using draviavemal.openxml_office.global_2007;
using openxml_office_fbs.document_2007;

namespace draviavemal.openxml_office.document_2007
{
    /// <summary>
    /// This class serves as a versatile tool for working with Word document
    /// Read Privacy Details document at https://openxml-office.draviavemal.com/privacy-policy
    /// </summary>
    public class Word : PrivacyProperties
    {
        private readonly IntPtr ffiWord;

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", CallingConvention = CallingConvention.Cdecl)]
        public static extern sbyte word_create(
            [MarshalAs(UnmanagedType.LPStr)] string optional_string,
            IntPtr buffer,
            int length,
            out IntPtr word,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", CallingConvention = CallingConvention.Cdecl)]
        public static extern sbyte word_save_as(
            IntPtr wordPtr,
            [MarshalAs(UnmanagedType.LPStr)] string file_name,
            out IntPtr error_msg
        );

        /// <summary>
        /// Create New file in the system
        /// Read Privacy Details document at https://openxml-office.draviavemal.com/privacy-policy
        /// </summary>
        public Word(WordProperties wordProperties)
        {
            FlatBufferBuilder builder = new(1024);
            DocumentPropertiesModel.StartDocumentPropertiesModel(builder);
            DocumentPropertiesModel.AddIsInMemory(builder, wordProperties.IsInMemory);
            Offset<DocumentPropertiesModel> wordPropertiesModel = DocumentPropertiesModel.EndDocumentPropertiesModel(builder);
            builder.Finish(wordPropertiesModel.Value);
            byte[] buffer = builder.SizedByteArray();
            int bufferSize = buffer.Length;
            unsafe
            {
                fixed (byte* bufferPtr = buffer)
                {
                    IntPtr bufferIntPtr = (IntPtr)bufferPtr;
                    sbyte statusCode = word_create(null, bufferIntPtr, bufferSize, out ffiWord, out IntPtr errorMsg);
                    StatusCode.ProcessStatusCode(statusCode, errorMsg);
                }
            }
        }

        /// <summary>
		/// Works with in memory object can be saved to file at later point.
		/// Source file will be cloned and released. hence can be replace by saveAs method if you want to update the same file.
		/// Read Privacy Details document at https://openxml-office.draviavemal.com/privacy-policy
		/// </summary>
        public Word(string fileName, WordProperties wordProperties)
        {
            FlatBufferBuilder builder = new(1024);
            DocumentPropertiesModel.StartDocumentPropertiesModel(builder);
            DocumentPropertiesModel.AddIsInMemory(builder, wordProperties.IsInMemory);
            Offset<DocumentPropertiesModel> wordPropertiesModel = DocumentPropertiesModel.EndDocumentPropertiesModel(builder);
            builder.Finish(wordPropertiesModel.Value);
            byte[] buffer = builder.SizedByteArray();
            int bufferSize = buffer.Length;
            unsafe
            {
                fixed (byte* bufferPtr = buffer)
                {
                    IntPtr bufferIntPtr = (IntPtr)bufferPtr;
                    sbyte statusCode = word_create(fileName, bufferIntPtr, bufferSize, out ffiWord, out IntPtr errorMsg);
                    StatusCode.ProcessStatusCode(statusCode, errorMsg);
                }
            }
        }

        /// <summary>
        /// Even on edit file OpenXML-Office Will clone the source and work on top of it to protect the integrity of source file.
        /// You can save the document at the end of lifecycle targeting the edit file to update or new file.
        /// This is supported for both file path and data stream
        /// </summary>
        public void SaveAs(string filePath)
        {
            sbyte statusCode = word_save_as(ffiWord, filePath, out IntPtr errorMsg);
            StatusCode.ProcessStatusCode(statusCode, errorMsg);
        }

    }
}
