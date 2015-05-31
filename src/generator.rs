use board::*;

pub fn get_rust_code(desc: &Description) -> String {

/*
    fn generateCode(in: Vec<Vec<i32>>) -> String {
        val field: Array[Int] = (in.flatten.filter(_ != -1))
    
        // calculate operations
        val output = new scala.collection.mutable.HashMap[Int, Long]
    
        for (i <- (field.length - 1) to 0 by -1) {
            val mask = 1L << i
    
            val e = field(field.length - 1 - i)
            val diff = e - i
    
            if (output contains diff) {
                output(diff) |= mask
            } else {
                output(diff) = mask
            }
        }
    
        // generate code
        val result = new StringBuilder
    
        result append "(\n   "
        var pos = 0
        for (i <- output) {
            result append "((f & "
            result append i._2
            result append "L)"
    
            if (i._1 > 0) {
                result append " << "
                result append math.abs(i._1)
            } else if (i._1 < 0) {
                result append " >> "
                result append math.abs(i._1)
            }
            result append ")"
    
            if (pos % 4 == 3)
                result append "\n"
    
            if (pos != output.size - 1)
                result append " | "
    
            pos += 1
        }
    
        result append ")"
    
        result.result
    }
    
    val get_normalform = new StringBuilder
    val get_equivalent_fields = new StringBuilder

    get_normalform append "def getNormalform(f: Long): Long = {\n"
    get_normalform append "var n = f\n\n"

    get_equivalent_fields append "def getEquivalentFields(f: Long) = {\n"
    get_equivalent_fields append "val n = new Array[Long](8)\n"
    get_equivalent_fields append "n(0) = f\n\n"

    if (is_transformation_valid(rotate180)) {
        val c_rotate180 = generateCode(rotate180(lookUpTable))
        get_normalform append "val n180 = " + c_rotate180 + "\n"
        get_normalform append "if(n180 < n) n = n180\n\n"

        get_equivalent_fields append "n(1) = " + c_rotate180 + "\n"
    }

    if (is_transformation_valid(rotate90)) {
        val c_rotate90 = generateCode(rotate90(lookUpTable))
        get_normalform append "val n90 = " + c_rotate90 + "\n"
        get_normalform append "if(n90 < n) n = n90\n\n"

        get_equivalent_fields append "n(2) = " + c_rotate90 + "\n"
    }

    if (is_transformation_valid(rotate270)) {
        val c_rotate270 = generateCode(rotate270(lookUpTable))
        get_normalform append "val n270 = " + c_rotate270 + "\n"
        get_normalform append "if(n270 < n) n = n270\n\n"

        get_equivalent_fields append "n(3) = " + c_rotate270 + "\n"
    }

    if (is_transformation_valid(vflip)) {
        val c_vflip = generateCode(vflip(lookUpTable))
        get_normalform append "val v = " + c_vflip + "\n"
        get_normalform append "if(v < n) n = v\n\n"

        get_equivalent_fields append "n(4) = " + c_vflip + "\n"
    }

    if (is_transformation_valid(hflip)) {
        val c_hflip = generateCode(hflip(lookUpTable))
        get_normalform append "val h = " + c_hflip + "\n"
        get_normalform append "if(h < n) n = h\n\n"

        get_equivalent_fields append "n(5) = " + c_hflip + "\n"
    }

    if (is_transformation_valid(vflip_rotate90)) {
        val c_v90 = generateCode(vflip_rotate90(lookUpTable))
        get_normalform append "val v90 = " + c_v90 + "\n"
        get_normalform append "if(v90 < n) n = v90\n\n"

        get_equivalent_fields append "n(6) = " + c_v90 + "\n"
    }

    if (is_transformation_valid(hflip_rotate90)) {
        val c_h90 = generateCode(hflip_rotate90(lookUpTable))
        get_normalform append "val h90 = " + c_h90 + "\n"
        get_normalform append "if(h90 < n) n = h90\n\n"

        get_equivalent_fields append "n(7) = " + c_h90 + "\n"
    }

    get_normalform append "n\n"
    get_normalform append "}\n"

    get_equivalent_fields append "n\n"
    get_equivalent_fields append "}\n"

    val r = new StringBuilder

    r append "result(0) = new com.googlecode.pegsolitaire.BoardHelper {\n"
    r append get_normalform.result
    r append get_equivalent_fields.result
    r append "}\n"

    r.result
*/
    
    String::new()
}