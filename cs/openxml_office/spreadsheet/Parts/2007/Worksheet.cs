
using System;

namespace draviavemal.openxml_office.spreadsheet_2007
{

    public class Worksheet
    {
        private readonly IntPtr ffiWorksheet;

        public Worksheet(IntPtr ffiWorksheet)
        {
            this.ffiWorksheet = ffiWorksheet;
        }
    }

}