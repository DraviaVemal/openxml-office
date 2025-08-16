// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.

using System.Collections.Generic;
namespace draviavemal.openxml_office.Global_2013
{
	/// <summary>
	/// Data Label Option Introduced at 2013 office upgrade
	/// </summary>
	public class AdvancedDataLabel
	{
		/// <summary>
		/// Determines whether to show the value from a column in the chart.
		/// </summary>
		public bool showValueFromColumn;
		/// <summary>
		/// Key For Data Column Value For Data Label Column If Data Label Column Are Present
		/// In between and Used in the list it will be auto skipped By Data Column
		/// </summary>
		public Dictionary<uint, uint> valueFromColumn = new Dictionary<uint, uint>();
	}
}
