using System;
using System.Runtime.InteropServices;
using Google.FlatBuffers;
using draviavemal.openxml_office.global_2007;
using openxml_office_fbs.document;

namespace draviavemal.openxml_office.document_2007
{
    /// <summary>
    /// This class serves as a versatile tool for working with Word document
    /// Read Privacy Details document at https://docs.draviavemal.com/openxml-office/privacy-policy
    /// </summary>
    public class Word : PrivacyProperties
    {
        private readonly ulong ffiWordPtr;

        /// <summary>
        /// 
        /// </summary>
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Word_create", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_word_create(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr out_buffer,
            out UIntPtr out_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Word_save_as", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_word_save_as(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr out_buffer,
            out UIntPtr out_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// Create New file in the system
        /// Read Privacy Details document at https://docs.draviavemal.com/openxml-office/privacy-policy
        /// </summary>
        public Word(WordProperties wordProperties = null)
        {
            ffiWordPtr = CreateWord(null, wordProperties);
        }

        /// <summary>
		/// Works with in memory object can be saved to file at later point.
		/// Source file will be cloned and released. hence can be replace by saveAs method if you want to update the same file.
		/// Read Privacy Details document at https://docs.draviavemal.com/openxml-office/privacy-policy
		/// </summary>
        public Word(string fileName, WordProperties wordProperties = null)
        {
            ffiWordPtr = CreateWord(fileName, wordProperties);
        }

        private static ulong CreateWord(string fileName, WordProperties wordProperties)
        {
            if (wordProperties == null)
            {
                wordProperties = new WordProperties();
            }
            FlatBufferBuilder builder = new(1024);
            StringOffset fbsFileName = fileName != null ? builder.CreateString(fileName) : default;
            Offset<Word_settings> fbsWordSettings = Word_settings.CreateWord_settings(builder, true);
            Offset<Word_create> fbsWordCreate = Word_create.CreateWord_create(builder, fbsFileName, fbsWordSettings);
            builder.Finish(fbsWordCreate.Value);
            byte[] responseBuffer = FfiInterop.InvokeBufferFfi(ffi_word_create, builder);
            Word_create_return response = Word_create_return.GetRootAsWord_create_return(new ByteBuffer(responseBuffer));
            return response.WordPtr;
        }

        /// <summary>
        /// Even on edit file OpenXML-Office Will clone the source and work on top of it to protect the integrity of source file.
        /// You can save the document at the end of lifecycle targeting the edit file to update or new file.
        /// This is supported for both file path and data stream
        /// </summary>
        public void SaveAs(string filePath)
        {
            FlatBufferBuilder builder = new(1024);
            StringOffset filePathOffset = builder.CreateString(filePath);
            Offset<Word_save_as> saveAsOffset = Word_save_as.CreateWord_save_as(builder, ffiWordPtr, filePathOffset);
            builder.Finish(saveAsOffset.Value);
            FfiInterop.InvokeBufferFfi(ffi_word_save_as, builder);
        }

    }
}
