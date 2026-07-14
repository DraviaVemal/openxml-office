
using System;

namespace draviavemal.openxml_office.spreadsheet_2007
{

    public class Worksheet
    {
        private readonly ulong ffiWorksheet;

        public Worksheet(ulong ffiWorksheet)
        {
            this.ffiWorksheet = ffiWorksheet;
        }
    }

}