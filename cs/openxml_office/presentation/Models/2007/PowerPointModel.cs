// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.

using System.Collections.Generic;
using draviavemal.openxml_office.global_2007;

namespace draviavemal.openxml_office.presentation_2007
{
	/// <summary>
	/// Represents the properties of a presentation.
	/// </summary>
	public class PowerPointProperties
	{
		/// <summary>
		/// 
		/// </summary>
		public bool IsInMemory = false;
		/// <summary>
		/// Gets or sets the presentation settings.
		/// </summary>
		public PowerPointSettings settings = new();
		/// <summary>
		/// Gets or sets the slide masters of the presentation.
		/// </summary>
		/// <remarks>
		/// TODO: Multi Theme Slide Master Support
		/// </remarks>
		public Dictionary<string, PowerPointSlideMaster> slideMasters;
		/// <summary>
		/// Gets or sets the theme of the presentation.
		/// </summary>
		public ThemePallet theme = new();
		/// <summary>
		/// Add Meta Data Details to File
		/// </summary>
		public CorePropertiesModel coreProperties = new();
	}
	/// <summary>
	/// Represents the settings of a presentation.
	/// </summary>
	public class PowerPointSettings
	{
		/// <summary>
		/// Gets or sets a value indicating whether the presentation has multiple slide masters.
		/// </summary>
		public bool isMultiSlideMasterPartPresentation;
		/// <summary>
		/// Gets or sets a value indicating whether the presentation has multiple themes.
		/// </summary>
		public bool isMultiThemePresentation;
	}
	/// <summary>
	/// Represents a slide master in a presentation.
	/// </summary>
	public class PowerPointSlideMaster
	{
		/// <summary>
		/// Gets or sets the theme of the slide master.
		/// </summary>
		public ThemePallet theme = new();
	}
	internal class PowerPointInfo
	{
		public bool isEditable = true;
		public bool isExistingFile;
	}
}
