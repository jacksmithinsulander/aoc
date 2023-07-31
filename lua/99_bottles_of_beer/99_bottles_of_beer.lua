beer = 99

while beer > 0 do
    verse = {}
    if beer > 2 then
        holder = " bottles"
        holder_b = " bottles"
    elseif beer == 1 then
        holder = " bottle"
        holder_b = " bottles"
    elseif beer == 2 then
        holder = " bottles"
        holder_b = " bottle"
    end
    table.insert(verse, beer)
    table.insert(verse, holder .. " of beer on the wall, \n")
    table.insert(verse, beer)
    table.insert(verse, holder .. " of beer!\nTake one down, pass it around\n")
    table.insert(verse, (beer - 1))
    table.insert(verse, holder_b .. " of beer on the wall \n")
    local song = table.concat(verse)
    print(song)
    beer = beer - 1
end