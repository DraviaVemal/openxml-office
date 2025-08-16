// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.

using draviavemal.openxml_office.global_2007;

namespace draviavemal.openxml_office.document_2007
{
	/// <summary>
	/// Represents the properties of a document.
	/// </summary>
	public class WordProperties
	{
		/// <summary>
		/// 
		/// </summary>
		public bool IsInMemory = false;
		/// <summary>
		/// Gets or sets the document settings.
		/// </summary>
		public WordSettings settings = new();
		/// <summary>
		/// Gets or sets the theme of the document.
		/// </summary>
		public ThemePallet theme = new();
		/// <summary>
		/// Add Meta Data Details to File
		/// </summary>
		public CorePropertiesModel coreProperties = new();
	}
	/// <summary>
	/// Represents the settings of a document.
	/// </summary>
	public class WordSettings { }

	internal class WordInfo
	{
		public bool isEditable = true;
		public bool isExistingFile;
	}
}
